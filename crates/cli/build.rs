// ref: https://github.com/mxre/winres/blob/master/example/build.rs
fn main() {
    // only run if target os is windows
    if std::env::var("CARGO_CFG_TARGET_OS")
        .map(|var| var != "windows")
        .unwrap_or(true)
    {
        return;
    }

    // only build the resource for release builds as calling rc.exe might be slow
    if std::env::var("PROFILE")
        .map(|var| var.starts_with("release"))
        .unwrap_or_default()
    {
        let mut res = winres::WindowsResource::new();
        if cfg!(unix) {
            res.set_toolkit_path("/usr/x86_64-w64-mingw32/bin"); // paths for X64 on Arch Linux
            res.set_ar_path("ar"); // ar tool for mingw in toolkit path
            res.set_windres_path("/usr/bin/x86_64-w64-mingw32-windres");
        }

        res.set("ProductName", env!("CARGO_PKG_NAME"))
            .set("FileDescription", env!("CARGO_PKG_DESCRIPTION"))
            .set("CompanyName", env!("CARGO_PKG_AUTHORS"))
            .set("LegalCopyright", env!("CARGO_PKG_AUTHORS"))
            .set_language(0x0409) // English	0x0009 / English (US)	0x0409
            .set_icon("assets/icon.ico")
            .set_manifest_file("assets/manifest.xml");

        if let Err(e) = res.compile() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
