fn main() {
    #[cfg(feature = "kf")]
    {
        let crate_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        // e.g., "x86_64-pc-windows-msvc"
        // let target_triple = std::env::var("TARGET").expect("TARGET env var is not set");
        let lib_root = crate_root.join("cpp_niflib");

        cxx_build::bridge("src/kf/from_kf/bridge.rs")
            .include(&lib_root)
            .include(lib_root.join("include"))
            .file(lib_root.join("wrapper.cc"))
            .compile("cxxbridge-fromkf");
        cxx_build::bridge("src/kf/to_kf/bridge.rs")
            .include(&lib_root)
            .include(lib_root.join("include"))
            .file(lib_root.join("wrapper_write.cc"))
            .compile("cxxbridge-tokf");

        println!("cargo:rerun-if-changed=wrapper.cc");
        println!("cargo:rerun-if-changed=wrapper.h");

        println!("cargo:rerun-if-changed=wrapper_write.cc");
        println!("cargo:rerun-if-changed=wrapper_write.h");

        println!("cargo:rustc-link-search={}", lib_root.display());
        println!("cargo:rustc-link-lib=static=niflib_static");
    }
}
