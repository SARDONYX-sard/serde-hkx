use super::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkBaseObject {
    _name: Option<Pointer>,
}

impl HavokClass for HkBaseObject {
    #[inline]
    fn name(&self) -> &'static CStr {
        c"hkBaseObject"
    }

    #[inline]
    fn signature(&self) -> Signature {
        Signature::new(0xea7f1d08)
    }
}
impl Serialize for HkBaseObject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let class_meta = self._name.map(|name| (name, self.signature()));
        let mut ser = serializer.serialize_struct("hkBaseObject", class_meta)?;
        ser.pad_field([0; 1].as_slice(), [0; 1].as_slice())?; // To vtable ptr size

        ser.end()
    }
}
