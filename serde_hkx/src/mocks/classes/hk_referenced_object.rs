use super::class::*;
use super::hk_base_object::HkBaseObject;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkReferencedObject {
    pub parent: HkBaseObject,

    pub _name: Option<Pointer>,

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

impl HavokClass for HkReferencedObject {}
impl Serialize for HkReferencedObject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use havok_serde::ser::SerializeStruct;

        let class_meta = self._name.map(|name| (name, Signature::new(0xea7f1d08)));
        let mut serializer = serializer.serialize_struct("hkReferencedObject", class_meta)?;

        serializer.skip_field("referenceCount", &self.reference_count)?;
        serializer.skip_field("memSizeAndFlags", &self.mem_size_and_flags)?;
        serializer.end()
    }
}
