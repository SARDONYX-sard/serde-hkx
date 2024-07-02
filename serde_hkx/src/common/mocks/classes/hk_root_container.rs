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
    fn name(&self) -> &'static str {
        "hkRootLevelContainer"
    }

    fn signature(&self) -> Signature {
        Signature::new(0x2772c11e)
    }
}

impl Serialize for HkRootLevelContainer<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let class_meta = self._name.map(|name| (name, self.signature()));
        let mut serializer = serializer.serialize_struct("hkRootLevelContainer", class_meta)?;

        // For XML & binary
        serializer.serialize_array_meta_field("namedVariants", &self.named_variants)?;

        // For binary
        serializer.serialize_array_field("namedVariants", &self.named_variants)?;
        serializer.end()
    }
}
