use super::*;

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

impl HavokClass for HkReferencedObject {
    fn name(&self) -> &'static CStr {
        c"hkReferencedObject"
    }

    fn signature(&self) -> Signature {
        Signature::new(0xea7f1d08)
    }
}

impl Serialize for HkReferencedObject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let class_meta = self._name.map(|name| (name, self.signature()));
        let mut serializer = serializer.serialize_struct("hkReferencedObject", class_meta)?;

        serializer.pad_field([0u8; 4].as_slice(), [0u8; 8].as_slice())?; // hkBaseObject ptr size
        serializer.skip_field("memSizeAndFlags", &self.mem_size_and_flags)?;
        serializer.skip_field("referenceCount", &self.reference_count)?;
        serializer.pad_field(&[0u8; 0].as_slice(), &[0u8; 4].as_slice())?; // tailing align by ptr size bytes
        serializer.end()
    }
}
