use super::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkRootLevelContainer<'a> {
    pub _name: Option<Pointer>,

    /// # C++ Class Fields Info
    /// -   name:`"namedVariants"`
    /// -   type: `hkArray<struct hkRootLevelContainerNamedVariant>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub named_variants: Vec<HkRootLevelContainerNamedVariant<'a>>,
}

impl HavokClass for HkRootLevelContainer<'_> {
    fn name(&self) -> &'static CStr {
        c"HkRootLevelContainer"
    }

    fn signature(&self) -> Signature {
        Signature::new(0xea7f1d08)
    }
}

impl Serialize for HkRootLevelContainer<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let class_meta = self._name.map(|name| (name, Signature::new(0xea7f1d08)));
        let mut serializer = serializer.serialize_struct("HkRootLevelContainer", class_meta)?;

        serializer.serialize_array_meta_field("namedVariants", &self.named_variants)?;
        serializer.end()
    }
}
