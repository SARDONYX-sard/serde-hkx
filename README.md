# Havok behavior Serialize/Deserialize library

## Features

- [x] 32bit to 64bit (reverse) conversion of hkx.
- [x] XML to 32bit/64bit hkx (reverse) conversion.
- [x] Display of hkx/XML state machine dependency tree.
- [x] Output logs that make it possible to determine the binary data location of hkx.
- [x] Hexdump the binary data of hkx.
- [x] Display data differences between two hkx/XML.

## Usage(For user)

[CLI release page](https://github.com/SARDONYX-sard/serde-hkx/releases)

## Usage(As a library)

- Convenience wrapper API(For CLI/GUI): (e.g. [CLI](./crates/cli/src/args/mod.rs))

```toml
# in Cargo.toml
serde_hkx_features = { git = "https://github.com/SARDONYX-sard/serde-hkx", tag = "0.2.0" }
```

- Low level API: (e.g. [Tests](./serde_hkx/src/tests/verify.rs))

```toml
# in Cargo.toml
havok_classes = { git = "https://github.com/SARDONYX-sard/serde-hkx", tag = "0.2.0" }
serde_hkx = { git = "https://github.com/SARDONYX-sard/serde-hkx", tag = "0.2.0" }
```

## Documentation

- Specification: [specification](./docs/specification/hkx_binary_format.md)
- Binary analysis: [hkx spec](./docs/handson_hex_dump/defaultmale/readme.md)

- API:

```shell
git clone https://github.com/SARDONYX-sard/serde-hkx.git;cd ./serde-hkx; cargo doc -p serde_hkx_features;
```

## Implementations processing

The only way to find out if it is possible or not is to try it out.

- [x] XML Serializer
- [x] XML Deserializer
- [x] Bytes Serializer
- [x] Bytes Deserializer
- [x] Generate Havok Classes
- [x] Testing the API & CI.
- [x] Modify the code based on the test data.

## About the GPL old version

GPL-dependent code (`rhexdump` crate) has been removed, and the GPL usage history has been completely removed.

- [Prev GPL History(ver. 0.0.0)](https://github.com/SARDONYX-sard/serde-hkx/releases/tag/0.0.0-prev-gpl-history)
