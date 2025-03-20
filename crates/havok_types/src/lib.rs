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
    ($($name:ident: $type:ty),+ $(,)?) => {
        $(
            #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
            #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            /// Represents either a primitive value, an event ID, or a variable ID.
            pub enum $name<'a> {
                /// Normal primitive value that does not require replacement.
                Number($type),
                /// Event ID pointing to the index of `eventNames` in `hkbBehaviorGraphStringData`.
                /// e.g., `$eventName[IdName]$`
                EventId(std::borrow::Cow<'a, str>),
                /// Variable ID pointing to the index of `variableNames` in `hkbBehaviorGraphStringData`.
                /// e.g., `$variableName[IdName]$`
                VariableId(std::borrow::Cow<'a, str>),
            }

            impl Default for $name<'_> {
                #[inline]
                fn default() -> Self {
                    Self::Number(Default::default())
                }
            }

            impl<'a> TryFrom<&'a str> for $name<'a> {
                type Error = String;

                fn try_from(s: &'a str) -> Result<Self, Self::Error> {
                    use std::borrow::Cow;
                    if let Ok(num) = s.parse::<$type>() {
                        Ok(Self::Number(num))
                    } else if let Some(captures) = s.strip_prefix("$eventName[").and_then(|s| s.strip_suffix("]$")) {
                        Ok(Self::EventId(Cow::Borrowed(captures)))
                    } else if let Some(captures) = s.strip_prefix("$variableName[").and_then(|s| s.strip_suffix("]$")) {
                        Ok(Self::VariableId(Cow::Borrowed(captures)))
                    } else {
                        Err(format!("Expected `$eventName[IdName]$`/`$variableName[IdName]$`. but got invalid string: {s}"))
                    }
                }
            }

            impl<'a> TryFrom<String> for $name<'a> {
                type Error = String;

                fn try_from(s: String) -> Result<Self, Self::Error> {
                    use std::borrow::Cow;
                    if let Ok(num) = s.parse::<$type>() {
                        Ok(Self::Number(num))
                    } else if let Some(captures) = s.strip_prefix("$eventName[").and_then(|s| s.strip_suffix("]$")) {
                        Ok(Self::EventId(Cow::Owned(captures.to_string())))
                    } else if let Some(captures) = s.strip_prefix("$variableName[").and_then(|s| s.strip_suffix("]$")) {
                        Ok(Self::VariableId(Cow::Owned(captures.to_string())))
                    } else {
                        Err(format!("Expected `$eventName[IdName]$`/`$variableName[IdName]$`. but got invalid string: {s}"))
                    }
                }
            }

            impl<'a> ::core::fmt::Display for $name<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        Self::Number(n) => write!(f, "{}", n),
                        Self::EventId(e) => write!(f, "$eventName[{}]$", e),
                        Self::VariableId(v) => write!(f, "$variableName[{}]$", v),
                    }
                }
            }

            #[cfg(feature = "serde")]
            impl<'a> serde::Serialize for $name<'a> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    serializer.serialize_str(&self.to_string())
                }
            }

            #[cfg(feature = "serde")]
            impl<'de> serde::Deserialize<'de> for $name<'de> {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    use std::borrow::Cow;
                    let s: Cow<'de, str> = Cow::deserialize(deserializer)?;
                    match s {
                        Cow::Borrowed(s) => Self::try_from(s).map_err(serde::de::Error::custom),
                        Cow::Owned(s) => Self::try_from(s).map_err(serde::de::Error::custom),
                    }
                }
            }
        )*
    };
}

create_enum! [
     U8: u8,
    U16: u16,
    U32: u32,
    U64: u64,
     I8: i8,
    I16: i16,
    I32: i32,
    I64: i64,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_number() {
        assert_eq!(U32::try_from("42"), Ok(U32::Number(42)));
        assert_eq!(I16::try_from("-10"), Ok(I16::Number(-10)));
    }

    #[test]
    fn test_from_str_event() {
        assert_eq!(
            U32::try_from("$eventName[Start]$"),
            Ok(U32::EventId("Start".into()))
        );
        assert_eq!(
            U32::try_from("$eventName[Jump]$"),
            Ok(U32::EventId("Jump".into()))
        );
    }

    #[test]
    fn test_from_str_variable() {
        assert_eq!(
            U32::try_from("$variableName[Health]$"),
            Ok(U32::VariableId("Health".into()))
        );
        assert_eq!(
            U32::try_from("$variableName[Stamina]$"),
            Ok(U32::VariableId("Stamina".into()))
        );
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(U32::try_from("random_text").is_err());
        assert!(U32::try_from("$invalidName[Oops]$").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() {
        let num = U32::Number(123);
        assert_eq!(serde_json::to_string(&num).unwrap(), "\"123\"");

        let event = U32::EventId("Fire".into());
        assert_eq!(
            serde_json::to_string(&event).unwrap(),
            "\"$eventName[Fire]$\""
        );

        let var = U32::VariableId("Health".into());
        assert_eq!(
            serde_json::to_string(&var).unwrap(),
            "\"$variableName[Health]$\""
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize() {
        let json = "\"123\"";
        let val: U32 = serde_json::from_str(json).unwrap();
        assert_eq!(val, U32::Number(123));

        let json = "\"$eventName[Jump]$\"";
        let event: U32 = serde_json::from_str(json).unwrap();
        assert_eq!(event, U32::EventId("Jump".into()));

        let json = "\"$variableName[Speed]$\"";
        let var: U32 = serde_json::from_str(json).unwrap();
        assert_eq!(var, U32::VariableId("Speed".into()));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_invalid_deserialize() {
        let json = "\"invalid_text\"";
        assert!(serde_json::from_str::<U32>(json).is_err());
    }
}
