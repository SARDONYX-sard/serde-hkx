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
    let svg_dpi = 96.0; // default: 96.0
    svg_to_ico::svg_to_ico(Path::new(&svg_path), svg_dpi, Path::new(&ico_path), &[16])?;

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
