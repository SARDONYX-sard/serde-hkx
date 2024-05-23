use havok_types::{Pointer, Signature};

use crate::ser::Serialize;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkReferencedObject {
    pub name: Option<Pointer>,

    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reference_count: i16,
}

impl crate::HavokClass for HkReferencedObject {}
impl Serialize for HkReferencedObject {
    fn serialize<S: crate::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use crate::ser::SerializeStruct;

        let class_meta = self.name.map(|name| (name, Signature::new(0xea7f1d08)));
        let mut serializer = serializer.serialize_struct("hkReferencedObject", class_meta)?;

        serializer.skip_field("referenceCount")?;
        serializer.skip_field("memSizeAndFlags")?;
        serializer.end()
    }
}
