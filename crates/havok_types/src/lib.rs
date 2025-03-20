//! All havok types definitions.
pub mod cstring;
pub mod half;
pub mod math;
pub mod pointer;
pub mod signature;
pub mod string_ptr;
pub mod ulong;
pub mod variant;

pub use cstring::*;
pub use half::f16;
pub use math::*;
pub use pointer::*;
pub use signature::*;
pub use string_ptr::*;
pub use ulong::*;
pub use variant::*;

mod lib {
    pub use std::borrow::Cow;

    pub use core::fmt;
    pub use core::fmt::Debug;
    pub use core::str::FromStr;
}

/// Unicode null
pub const NULL_STR: &str = "\u{2400}";

macro_rules! create_enum {
    ($($name:ident => $type:ty),*) => {
        $(
            #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, parse_display::Display)]
            #[display("{0}")]
            pub enum $name<'a> {
                Number($type),
                Var(std::borrow::Cow<'a, str>),
            }

            impl Default for $name<'_> {
                #[inline]
                fn default() -> Self {
                    Self::Number(Default::default())
                }
            }
        )*
    };
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, parse_display::Display)]
#[display("{0}")]
pub enum I64<'a> {
    Number(i64),
    Var(std::borrow::Cow<'a, str>),
}
impl Default for I64<'_> {
    #[inline]
    fn default() -> Self {
        Self::Number(Default::default())
    }
}

impl<'a> I64<'a> {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &'a str) -> Self {
        s.parse::<i64>()
            .map_or(Self::Var(std::borrow::Cow::Borrowed(s)), |num| {
                Self::Number(num)
            })
    }
}

create_enum! [
    U8 => u8,
    U16 => u16,
    U32 => u32,
    U64 => u64,

    I8 => i8,
    I16 => i16,
    I32 => i32
    // I64 => i64
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_type() {
        let array: [U8; 5] = core::array::from_fn(|i| U8::Number(i as u8));
        println!("{:?}", array);
    }
}
