# Havok behavior Serialize/Deserialize library

## Usage

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

## Manufacturing process

The only way to find out if it is possible or not is to try it out.

### Try implementations

- [x] Try to implement Serializer to design.
  - [x] XML
  - [x] Bytes
- [x] Try to implement Deserializer to design.

  - [x] XML
  - [x] Bytes

- [x] Generate Classes

- [ ] Modify the code based on the test data. <- Working in progress

### Implementations

- [ ] Analyze and document binary and XML read/write specifications for hkx files. (To avoid design omissions in the API.)
- [ ] Design the API. (To design an ideal API to prevent turnaround)
- [ ] Implement the API.
- [ ] Testing the API & CI.

## This repository has completely removed GPL-dependent code and its history

Previous GPL-tainted versions can be found at the following links.

- [0.0.0-prev-gpl-history](https://github.com/SARDONYX-sard/serde-hkx/releases/tag/0.0.0-prev-gpl-history)

GPL-dependent code (`rhexdump` crate) has been removed, and the GPL usage history has been completely removed.
