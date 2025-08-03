use std::env;
use std::path::Path;

fn main() {
    // only run if target os is windows
    let is_non_windows = env::var("CARGO_CFG_TARGET_OS")
        .map(|os| os != "windows")
        .unwrap_or(true);

    // only build the resource for release builds as calling rc.exe might be slow
    let is_release = env::var("PROFILE")
        .map(|p| p.starts_with("release"))
        .unwrap_or(false);

    if is_non_windows || !is_release {
        return;
    }

    if let Err(e) = embed_resources() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// ref: https://github.com/mxre/winres/blob/master/example/build.rs
fn embed_resources() -> Result<(), svg_to_ico::Error> {
    let mut res = winres::WindowsResource::new();

    #[cfg(unix)]
    {
        res.set_toolkit_path("/usr/x86_64-w64-mingw32/bin");
        res.set_ar_path("ar");
        res.set_windres_path("/usr/bin/x86_64-w64-mingw32-windres");
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let svg_path = format!("{manifest_dir}/assets/icon.svg");
    let ico_path = format!("{manifest_dir}/assets/icon.ico");
    let svg_dpi = 192.0; // default: 96.0
    svg_to_ico::svg_to_ico(Path::new(&svg_path), svg_dpi, Path::new(&ico_path), &[32])?;

    res.set("ProductName", env!("CARGO_PKG_NAME"))
        .set("FileDescription", env!("CARGO_PKG_DESCRIPTION"))
        .set("CompanyName", env!("CARGO_PKG_AUTHORS"))
        .set("LegalCopyright", env!("CARGO_PKG_AUTHORS"))
        .set_language(0x0409)
        .set_icon(&ico_path)
        .set_manifest_file("assets/manifest.xml");

    res.compile()?;
    Ok(())
}

/// svg_to_ico v1.2.0
/// - ref: https://github.com/Ortham/svg_to_ico/blob/master/LICENSE
///
/// SPDX-FileCopyrightText: (C) 2018 Oliver Hamlet
/// SPDX-License-Identifier: Apache-2.0 OR MIT
mod svg_to_ico {
    use std::fs::{File, create_dir_all, read};
    use std::io;
    use std::path::Path;

    use tiny_skia::Pixmap;
    use usvg::Tree;

    /// Error returned when creating an ICO file from an SVG file fails.
    #[derive(Debug)]
    #[allow(clippy::enum_variant_names)]
    pub enum Error {
        /// An I/O error occurred, e.g. the input file doesn't exist.
        IoError(std::io::Error),
        /// Something went wrong when parsing the SVG file.
        ParseError,
        /// Something went wrong when rasterizing the SVG file.
        RasterizeError,
    }

    impl From<std::io::Error> for Error {
        fn from(error: std::io::Error) -> Self {
            Self::IoError(error)
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match *self {
                Self::IoError(ref e) => e.fmt(f),
                Self::ParseError => write!(f, "An unknown SVG parsing error"),
                Self::RasterizeError => write!(f, "Failed to rasterize SVG"),
            }
        }
    }

    /// Create a new ICO file from given SVG file.
    ///
    /// SVG dimensions are interpreted as pixels and the image rasterized using the given DPI. The ICO
    /// entry sizes are the heights in pixels of the images to store inside the ICO file: the SVG image
    /// will be scaled to produce images of the specified sizes. If the ICO
    /// file's parent directory does not exist, it will be created.
    pub(crate) fn svg_to_ico(
        svg_path: &Path,
        svg_dpi: f32,
        ico_path: &Path,
        ico_entry_sizes: &[u16],
    ) -> Result<(), Error> {
        let opt = usvg::Options {
            dpi: svg_dpi,
            ..Default::default()
        };

        let file_content = read(svg_path)?;
        let svg = Tree::from_data(&file_content, &opt).map_err(|_| Error::ParseError)?;

        let images = ico_entry_sizes
            .iter()
            .map(|size| rasterize(&svg, *size))
            .collect::<Result<Vec<_>, Error>>()?;

        create_ico(ico_path, images).map_err(Error::from)
    }

    fn rasterize(svg: &Tree, height_in_pixels: u16) -> Result<Pixmap, Error> {
        let target_height: f32 = height_in_pixels.into();
        #[allow(clippy::expect_used)]
        let target_size = tiny_skia::Size::from_wh(target_height, target_height)
            .expect("Unsigned values should always be valid");

        let scaled_size = svg.size().scale_to(target_size);

        let sx = scaled_size.width() / svg.size().width();
        let sy = scaled_size.height() / svg.size().height();
        let transform = tiny_skia::Transform::from_scale(sx, sy);

        let pixmap_size = scaled_size.to_int_size();

        Pixmap::new(pixmap_size.width(), pixmap_size.height())
            .map(|mut pixmap| {
                let mut pixmap_mut = pixmap.as_mut();
                resvg::render(svg, transform, &mut pixmap_mut);
                pixmap
            })
            .ok_or(Error::RasterizeError)
    }

    fn create_ico(ico_path: &Path, pngs: Vec<Pixmap>) -> io::Result<()> {
        let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

        for png in pngs {
            let image = ico::IconImage::from_rgba_data(png.width(), png.height(), png.take());
            icon_dir.add_entry(ico::IconDirEntry::encode(&image)?);
        }

        if let Some(p) = ico_path.parent() {
            create_dir_all(p)?;
        }

        let file = File::create(ico_path)?;
        icon_dir.write(file)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn load_svg(path: &Path) -> Tree {
            let svg_dpi = 96.0;

            let opt = usvg::Options::<'_> {
                dpi: svg_dpi,
                ..Default::default()
            };

            let file_content = read(path).unwrap();
            Tree::from_data(&file_content, &opt).unwrap()
        }

        #[test]
        fn rasterize_should_scale_svg_to_given_height() {
            let svg_path = Path::new("examples/example.svg");
            let svg = load_svg(svg_path);

            assert_eq!(24.0, svg.size().height());
            assert_eq!(24.0, svg.size().width());

            let image = rasterize(&svg, 400).unwrap();
            assert_eq!(400, image.height());
            assert_eq!(400, image.width());
        }

        #[test]
        fn rasterize_should_set_pixel_colour_correctly() {
            let svg_path = Path::new("examples/example.svg");
            let svg = load_svg(svg_path);

            let image = rasterize(&svg, 24).unwrap();
            let pixel_index = 24 * 6 + 12;
            let pixel = &image.take()[pixel_index * 4..(pixel_index + 1) * 4];

            assert_eq!(&[50, 100, 150, 255], pixel);
        }

        #[test]
        fn rasterize_should_scale_svg_with_width_longer_than_height() {
            let svg_path = Path::new("examples/landscape.svg");
            let svg = load_svg(svg_path);

            assert_eq!(24.0, svg.size().height());
            assert_eq!(48.0, svg.size().width());

            let image = rasterize(&svg, 400).unwrap();
            assert_eq!(200, image.height());
            assert_eq!(400, image.width());
        }
    }
}
