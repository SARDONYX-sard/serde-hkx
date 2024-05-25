use super::class::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkBaseObject {
    _name: Option<Pointer>,
}

impl HavokClass for HkBaseObject {}
impl Serialize for HkBaseObject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use havok_serde::ser::SerializeStruct;

        let class_meta = self._name.map(|name| (name, Signature::new(0xea7f1d08)));
        let mut ser = serializer.serialize_struct("hkBaseObject", class_meta)?;
        ser.pad_field(&Pointer::new(0))?; // vtable pointer size
        ser.end()
    }
}