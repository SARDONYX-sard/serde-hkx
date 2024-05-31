use super::HkReferencedObject;
use crate::mocks::enums::EventMode;
use crate::mocks::mock_requires::*;

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

impl HavokClass for HkbProjectData {}
impl Serialize for HkbProjectData {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let class_meta = self._name.map(|name| (name, Signature::new(0xea7f1d08)));
        let mut serializer = serializer.serialize_struct("HkbProjectData", class_meta)?;

        serializer.serialize_field("worldUpWS", &self.world_up_ws)?;
        serializer.serialize_field("stringData", &self.string_data)?;
        serializer.serialize_field("defaultEventMode", &self.default_event_mode)?;
        serializer.end()
    }
}
