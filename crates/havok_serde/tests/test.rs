use havok_serde::{HavokClass as _, Serialize};

#[derive(Debug, Default, Serialize)]
#[havok_serde(x86_size = 2, x64_size = 2)]
#[havok_serde(name = "hkObject", signature = 0x111)]
struct Object {
    /// Class index of itself.
    ///
    /// In XML, this takes the place of a pointer.
    /// This index is generated when deserializing a binary file.
    ///
    /// # Note
    /// [`Option::None`] -> field within the class.
    __ptr_name_attr: Option<havok_types::Pointer>,
    #[havok_serde(rename = "First")]
    #[havok_serde(x86_offset = 0, x64_offset = 0)]
    field: u16,
}

#[test]
fn macro_test() {
    let obj = Object::default();
    dbg!(&obj);
    dbg!(obj.name());
    dbg!(obj.signature());
}
