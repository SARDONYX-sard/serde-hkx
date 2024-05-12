#[macro_export]
/// - [`serde::Serialize`] with [`core::fmt::Display`]
/// - [`serde::Deserialize`] with [`core::str::FromStr`]
///
/// # Examples
///
/// ```
/// #[derive(Debug, PartialEq, Eq)]
/// struct TestStruct {
///     value: i32,
/// }
///
/// /// To Serialize
/// impl core::fmt::Display for TestStruct {
///     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
///         write!(f, "{}", self.value)
///     }
/// }
///
/// /// To Deserialize
/// impl core::str::FromStr for TestStruct {
///     type Err = core::num::ParseIntError;
///
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         Ok(Self { value: s.parse()? })
///     }
/// }
///
/// impl_str_serde!(TestStruct);
///
/// // Serialize
/// let test_struct = TestStruct { value: 42 };
/// let serialized = serde_json::to_string(&test_struct).unwrap();
/// assert_eq!(serialized, "\"42\"");
///
/// // Deserialize
/// let deserialized: TestStruct = serde_json::from_str("\"42\"").unwrap();
/// assert_eq!(deserialized, test_struct);
/// ```
macro_rules! impl_str_serde {
    ($struct_name:ident) => {
        impl serde::Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> serde::Deserialize<'de> for $struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s: &str = serde::Deserialize::deserialize(deserializer)?;
                s.parse().map_err(|_| {
                    serde::de::Error::custom(format!(
                        "Invalid value: {s} for {}",
                        stringify!($struct_name)
                    ))
                })
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq, Eq)]
    struct TestStruct {
        value: i32,
    }

    impl core::fmt::Display for TestStruct {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl core::str::FromStr for TestStruct {
        type Err = core::num::ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self { value: s.parse()? })
        }
    }

    impl_str_serde!(TestStruct);

    #[test]
    fn serialize_deserialize_test() {
        // Serialize
        let test_struct = TestStruct { value: 42 };
        let serialized = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(serialized, "\"42\"");

        // Deserialize
        let deserialized: TestStruct = serde_json::from_str("\"42\"").unwrap();
        assert_eq!(deserialized, test_struct);
    }
}
