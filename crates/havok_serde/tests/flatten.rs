use havok_serde::{HavokClass as _, Serialize};
use havok_types::StringPtr;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[havok_serde(x86_size = 8, x64_size = 16)]
#[havok_serde(name = "hkaBone", signature = 0x35912f8a)]
struct HkaBone<'a> {
    /// Class index of itself.
    ///
    /// In XML, this takes the place of a pointer.
    /// This index is generated when deserializing a binary file.
    ///
    /// # Note
    /// The case of [`Option::None`] assumes that the class is embedded directly in a field within the class.
    __ptr_name_attr: Option<havok_types::Pointer>,

    #[havok_serde(rename = "StringPtr")]
    #[havok_serde(x86_offset = 0, x64_offset = 0)]
    #[havok_serde(x86_type_size = 4, x64_type_size = 8)]
    name: StringPtr<'a>,

    #[havok_serde(rename = "StringPtr")]
    #[havok_serde(x86_offset = 4, x64_offset = 8)]
    #[havok_serde(x86_type_size = 1, x64_type_size = 1)]
    lock_translation: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[havok_serde(x86_size = 10, x64_size = 18)]
#[havok_serde(name = "hkChild", signature = 0xea7f1d08)]
struct Child<'a> {
    __ptr_name_attr: Option<havok_types::Pointer>,

    #[havok_serde(flatten = "hkaBone")]
    __parent: HkaBone<'a>,

    #[havok_serde(rename = "Child")]
    #[havok_serde(x86_offset = 8, x64_offset = 16)]
    #[havok_serde(x86_type_size = 2, x64_type_size = 2)]
    child: u16,
}

#[test]
fn macro_test() {
    let obj = Child::default();
    dbg!(&obj);
    dbg!(obj.name());
    dbg!(obj.signature());
}
