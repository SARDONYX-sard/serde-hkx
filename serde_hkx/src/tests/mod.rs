mod mermaid;
pub(crate) mod mocks;
mod parse_skyrim;

/// - key: class index(e.g. `#0001`)
/// - value: class
pub(crate) type ClassMap<'a> = indexmap::IndexMap<usize, havok_classes::Classes<'a>>;
