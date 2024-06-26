use havok_serde::{HavokClass as _, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[havok_serde(x86_size = 8, x64_size = 16)]
#[havok_serde(name = "hkReferencedObject", signature = 0xea7f1d08)]
pub struct HkReferencedObject {
    /// Class index of itself.
    ///
    /// In XML, this takes the place of a pointer.
    /// This index is generated when deserializing a binary file.
    ///
    /// # Note
    /// The case of [`Option::None`] assumes that the class is embedded directly in a field within the class.
    __ptr_name_attr: Option<havok_types::Pointer>,

    /// fe
    #[havok_serde(rename = "memSizeAndFlags")]
    #[havok_serde(x86_offset = 4, x64_offset = 8)]
    #[havok_serde(x86_type_size = 2, x64_type_size = 2, skip_serializing)]
    mem_size_and_flags: u16,

    /// fe
    #[havok_serde(rename = "referenceCount")]
    #[havok_serde(x86_offset = 6, x64_offset = 10)]
    #[havok_serde(x86_type_size = 2, x64_type_size = 2, skip_serializing)]
    reference_count: i16,
}

#[test]
fn macro_test() {
    let obj = HkReferencedObject::default();
    dbg!(&obj);
    dbg!(obj.name());
    dbg!(obj.signature());
}
