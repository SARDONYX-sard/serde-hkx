//! Binary parser combinator
pub mod fixups;
pub mod type_kind;

pub type BytesStream<'a> = &'a [u8];
