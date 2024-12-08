# Havok Behavior Serialization/Deserialization Library

<div>
    <a href="https://serde-hkx-api.netlify.app/serde_hkx_features">
        <img src="https://api.netlify.com/api/v1/badges/5f3f47f7-b22b-4d0d-b914-dd697b444858/deploy-status" alt="API docs">
    </a>
    <a href="https://github.com/SARDONYX-sard/serde-hkx/actions/workflows/release-cli.yaml">
        <img src="https://github.com/SARDONYX-sard/serde-hkx/actions/workflows/release-cli.yaml/badge.svg" alt="Release(CLI)">
    </a>
    <a href="https://github.com/SARDONYX-sard/serde-hkx/actions/workflows/build-and-test.yaml">
        <img src="https://github.com/SARDONYX-sard/serde-hkx/actions/workflows/build-and-test.yaml/badge.svg" alt="Build & Test(Cargo)">
    </a>
    <a href="https://github.com/SARDONYX-sard/serde-hkx">
        <img src="https://tokei.rs/b1/github/SARDONYX-sard/serde-hkx?category=lines" alt="Line of Code">
    </a>
    <a href="https://opensource.org/licenses/MIT">
        <img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License">
    </a>
    <a href="https://opensource.org/licenses/Apache-2.0">
        <img src="https://img.shields.io/badge/License-Apache_2.0-blue.svg" alt="License">
    </a>
</div>

## Features

- [x] 32bit to 64bit (reverse) conversion of hkx.
- [x] XML to 32bit/64bit hkx (reverse) conversion.
- [x] Display of hkx/XML state machine dependency tree.
- [x] Output logs that make it possible to determine the binary data location of
      hkx.
- [x] Hexdump the binary data of hkx.
- [x] Display data differences between two hkx/XML.

## Download CLI(For behavior creator)

- [CLI release page](https://github.com/SARDONYX-sard/serde-hkx/releases)

- [Nexus](https://www.nexusmods.com/skyrimspecialedition/mods/126214/?tab=files)

```shell
./hkxc --help
```

## When used as a library(For developer)

NOTE: Currently there is a stack overflow problem of unknown cause. This occurs
with debug build (`cargo build`) but not with release (`cargo build --release`).

- Convenience wrapper API(For CLI/GUI):
  [Examples](./crates/cli/src/args/mod.rs)/[API docs](https://serde-hkx-api.netlify.app/serde_hkx_features/)

```toml
# in Cargo.toml
tokio = { version = "1.41.0", features = ["full"] } # Async runtime
serde_hkx_features = { git = "https://github.com/SARDONYX-sard/serde-hkx", tag = "0.5.0" }
```

<!--
no_run + rust syntax highlighting
- ref: https://www.reddit.com/r/rust/comments/pl589v/how_to_ignore_code_blocks_in_readme_with_doc/
-->

```rust ,no_run
use serde_hkx_features::convert::{convert, OutFormat};
use serde_hkx_features::error::{Result};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
  let input = "./defaultmale.hkx"; // file or dir path
  let out_fmt = OutFormat::from_input(&input)?; // `.hkx` -> OutFormat::Xml, `.xml` -> OutFormat::Amd64
  let output: Option<PathBuf> = None; // `None` is same as input. Or `Some("./output/defaultmale.xml".into())`
  convert(input, output, out_fmt).await
}
```

- Low level API:
  [Tests](./serde_hkx/src/tests/verify.rs)/[API docs](https://serde-hkx-api.netlify.app/serde_hkx/)

```toml
# in Cargo.toml
havok_classes = { git = "https://github.com/SARDONYX-sard/serde-hkx", tag = "0.5.0" }
serde_hkx = { git = "https://github.com/SARDONYX-sard/serde-hkx", tag = "0.5.0" }
```

## Documentation

- [Specification](./docs/specification/hkx_binary_format.md)
- [Binary analysis](./docs/handson_hex_dump/defaultmale/readme.md)
- [API](https://serde-hkx-api.netlify.app/serde_hkx/)

## implementation progress

The only way to find out if it is possible or not is to try it out.

- [x] XML Serializer
- [x] XML Deserializer
- [x] Bytes Serializer
- [x] Bytes Deserializer
- [x] Generate Havok Classes
- [x] Testing the API & CI.
- [x] Modify the code based on the test data.
