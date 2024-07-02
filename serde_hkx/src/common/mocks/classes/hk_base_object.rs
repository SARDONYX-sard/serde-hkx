use super::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkBaseObject {
    pub _name: Option<Pointer>,
}

impl HavokClass for HkBaseObject {
    #[inline]
    fn name(&self) -> &'static str {
        "hkBaseObject"
    }

    #[inline]
    fn signature(&self) -> Signature {
        Signature::new(0xea7f1d08)
    }
}
impl Serialize for HkBaseObject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let class_meta = self._name.map(|name| (name, self.signature()));
        let mut serializer = serializer.serialize_struct("hkBaseObject", class_meta)?;

        serializer.pad_field([0u8; 4].as_slice(), [0u8; 8].as_slice())?; // hkBaseObject ptr size
        serializer.end()
    }
}
