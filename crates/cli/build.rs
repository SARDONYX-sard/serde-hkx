use std::env;
use std::fs::File;
use std::io::BufWriter;
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

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let svg_path = format!("{manifest_dir}/assets/icon.svg");
    let ico_path = format!("{manifest_dir}/assets/icon.ico");

    let embed_res = || -> Result<(), Box<dyn std::error::Error>> {
        generate_icon(&svg_path, &ico_path)?;
        embed_resources(&ico_path)?;
        Ok(())
    };
    if let Err(e) = embed_res() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn generate_icon(svg_path: &str, ico_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use ico::{IconDir, IconImage};
    use resvg::tiny_skia::{Pixmap, Transform};
    use resvg::usvg::{Options, Tree};

    let sizes = [16];
    // let sizes = [16, 32, 48, 64, 128, 256];

    let svg_data = std::fs::read(svg_path)?;
    let mut icon_dir = IconDir::new(ico::ResourceType::Icon);

    for size in sizes {
        // Load SVG
        let opt = Options {
            resources_dir: Some(
                Path::new(svg_path)
                    .parent()
                    .ok_or("Failed to get resources dir")?
                    .to_path_buf(),
            ),
            ..Default::default()
        };
        let r_tree = Tree::from_data(&svg_data, &opt)?;

        // Render
        let mut pixmap = Pixmap::new(size, size).ok_or("Pixel must be non-zero size")?;
        let scale = Transform::from_scale(size as f32, size as f32);
        resvg::render(&r_tree, scale, &mut pixmap.as_mut());

        // Convert Pixmap to PNG in memory
        let png_data = pixmap.encode_png()?;
        let image = IconImage::read_png(png_data.as_slice())?;
        let image = ico::IconDirEntry::encode_as_png(&image)?;
        icon_dir.add_entry(image);
    }

    let mut file = BufWriter::new(File::create(ico_path)?);
    icon_dir.write(&mut file)?;

    Ok(())
}

// ref: https://github.com/mxre/winres/blob/master/example/build.rs
fn embed_resources(icon_path: &str) -> std::io::Result<()> {
    let mut res = winres::WindowsResource::new();

    {
        res.set_toolkit_path("/usr/x86_64-w64-mingw32/bin");
        res.set_ar_path("ar");
        res.set_windres_path("/usr/bin/x86_64-w64-mingw32-windres");
    }

    res.set("ProductName", env!("CARGO_PKG_NAME"))
        .set("FileDescription", env!("CARGO_PKG_DESCRIPTION"))
        .set("CompanyName", env!("CARGO_PKG_AUTHORS"))
        .set("LegalCopyright", env!("CARGO_PKG_AUTHORS"))
        .set_language(0x0409)
        .set_icon(icon_path)
        .set_manifest_file("assets/manifest.xml");

    res.compile()
}
