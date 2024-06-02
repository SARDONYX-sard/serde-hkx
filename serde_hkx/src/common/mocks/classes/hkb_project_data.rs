use super::*;
use crate::common::mocks::enums::EventMode;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbProjectData {
    pub parent: HkReferencedObject,

    pub _name: Option<Pointer>,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"worldUpWS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub world_up_ws: Vector4,
    /// # C++ Class Fields Info
    /// -   name:`"stringData"`
    /// -   type: `struct hkbProjectStringData*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub string_data: Pointer,
    /// # C++ Class Fields Info
    /// -   name:`"defaultEventMode"`
    /// -   type: `enum EventMode`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub default_event_mode: EventMode,
}

impl HavokClass for HkbProjectData {
    fn name(&self) -> &'static CStr {
        c"hkbProjectData"
    }

    fn signature(&self) -> Signature {
        Signature::new(0xea7f1d08)
    }
}

impl Serialize for HkbProjectData {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let class_meta = self._name.map(|name| (name, self.signature()));
        let mut serializer = serializer.serialize_struct("hkbProjectData", class_meta)?;

        // flattened parent's fields
        serializer.pad_field([0u8; 4].as_slice(), [0u8; 8].as_slice())?; // hkBaseObject ptr size
        serializer.skip_field("referenceCount", &self.parent.reference_count)?;
        serializer.skip_field("memSizeAndFlags", &self.parent.mem_size_and_flags)?;
        serializer.pad_field(&[0u8; 0].as_slice(), &[0u8; 4].as_slice())?;

        serializer.serialize_field("worldUpWS", &self.world_up_ws)?;
        serializer.serialize_field("stringData", &self.string_data)?;
        serializer.serialize_field("defaultEventMode", &self.default_event_mode)?;
        serializer.pad_field(&[0u8; 11].as_slice(), &[0u8; 7].as_slice())?; // tailing alignment for struct
        serializer.end()
    }
}
