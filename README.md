# Havok behavior Serialize/Deserialize library

## Usage(For user)

[CLI release page](https://github.com/SARDONYX-sard/serde-hkx/releases)

The following functions can be attempted using the CLI.

- 32bit to 64bit (reverse) conversion of hkx.
- XML to 32bit/64bit hkx (reverse) conversion.
- Display of hkx/XML state machine fiber tree.
- Output logs that make it possible to determine the binary data location of hkx.
- hexdump the binary data of hkx.
- Display data differences between two hkx/XML.

## Usage(As library)

- Cargo.toml

```toml
havok_classes = { git = "https://github.com/SARDONYX-sard/serde-hkx" }
havok_serde = { git = "https://github.com/SARDONYX-sard/serde-hkx" }
havok_types = { git = "https://github.com/SARDONYX-sard/serde-hkx" }
serde_hkx = { git = "https://github.com/SARDONYX-sard/serde-hkx" }
```

Examples

- [Tests](./serde_hkx/src/tests/verify.rs)
- [Convert(in cli)](./crates/cli/src/cli/convert.rs)

## Try implementations

The only way to find out if it is possible or not is to try it out.

- [x] Try to implement Serializer to design.
  - [x] XML
  - [x] Bytes
- [x] Try to implement Deserializer to design.

  - [x] XML
  - [x] Bytes

- [x] Generate Classes

- [x] Testing the API & CI.
- [ ] Modify the code based on the test data. <- (Finally, only global_fixup order issues...)

## About the GPL old version

GPL-dependent code (`rhexdump` crate) has been removed, and the GPL usage history has been completely removed.
Previous GPL-tainted versions can be found at the following links.

- [Prev GPL History(ver. 0.0.0)](https://github.com/SARDONYX-sard/serde-hkx/releases/tag/0.0.0-prev-gpl-history)
