#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

fn main() {
    #[cfg(feature = "kf")]
    {
        let crate_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        // e.g., "x86_64-pc-windows-msvc"
        let target_triple = std::env::var("TARGET").expect("TARGET env var is not set");
        let lib_root = crate_root.join("cpp_niflib");
        let include_dir = lib_root.join("include");
        let lib_dir_path = lib_root.join("lib");

        // Download C++ libraries
        let libs_existed = std::fs::exists(&include_dir).unwrap_or_default();
        let lib_dir_existed = std::fs::exists(&lib_dir_path).unwrap_or_default();
        if !libs_existed || !lib_dir_existed {
            fetch_libs(&lib_root, &target_triple);
        }

        cxx_build::bridge("src/kf/from_kf/bridge.rs")
            .include(&lib_root)
            .include(&include_dir)
            .file(lib_root.join("wrapper.cc"))
            .compile("cxxbridge-fromkf");
        cxx_build::bridge("src/kf/to_kf/bridge.rs")
            .include(&lib_root)
            .include(&include_dir)
            .file(lib_root.join("wrapper_write.cc"))
            .compile("cxxbridge-tokf");

        println!("cargo:rerun-if-changed=wrapper.cc");
        println!("cargo:rerun-if-changed=wrapper.h");

        println!("cargo:rerun-if-changed=wrapper_write.cc");
        println!("cargo:rerun-if-changed=wrapper_write.h");

        println!("cargo:rustc-link-search={}", lib_root.join("lib").display());
        println!("cargo:rustc-link-lib=static=niflib_static");
    }
}

#[cfg(feature = "kf")]
fn fetch_libs<P>(out_dir: P, target_triple: &str)
where
    P: AsRef<std::path::Path>,
{
    use std::io::Cursor;

    let url = format!(
        "https://github.com/SARDONYX-sard/niflib_ffi/releases/download/cpp/niflib_{target_triple}.zip",
    );
    let out_dir = out_dir.as_ref();

    // Download zip(Wait up to 30 minutes to download 160 MB considering the slow network.)
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60 * 30))
        .build()
        .unwrap();
    let response = client
        .get(&url)
        .send()
        .unwrap_or_else(|e| panic!("Failed to download ZIP. url: {url}, err: {e}"));
    let bytes = response.bytes().expect("Failed to read response bytes");

    let mut archive =
        zip::read::ZipArchive::new(Cursor::new(bytes)).unwrap_or_else(|err| panic!("{err}"));
    archive
        .extract(out_dir)
        .unwrap_or_else(|err| panic!("{err}"));
}
