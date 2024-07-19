# Havok Animation Serialize/Deserialize library

## Usage

- Cargo.toml

```toml
havok_classes = { git = "https://github.com/SARDONYX-sard/serde-hkx", rev = "d8a6ca7da611cb03d3aec913db77ebc1b2a43d82" }
havok_serde = { git = "https://github.com/SARDONYX-sard/serde-hkx", rev = "d8a6ca7da611cb03d3aec913db77ebc1b2a43d82" }
havok_types = { git = "https://github.com/SARDONYX-sard/serde-hkx", rev = "d8a6ca7da611cb03d3aec913db77ebc1b2a43d82" }
serde_hkx = { git = "https://github.com/SARDONYX-sard/serde-hkx", rev = "d8a6ca7da611cb03d3aec913db77ebc1b2a43d82" }
```

See [tests](./serde_hkx/tests/parse_skyrim.rs)

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
