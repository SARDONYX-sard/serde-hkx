use super::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkRootLevelContainerNamedVariant<'a> {
    pub _name: Option<Pointer>,

    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub name: StringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"className"`
    /// -   type: `hkStringPtr`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub class_name: StringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"variant"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub variant: Pointer,
}

impl HavokClass for HkRootLevelContainerNamedVariant<'_> {
    fn name(&self) -> &'static CStr {
        c"hkRootLevelContainerNamedVariant"
    }

    fn signature(&self) -> Signature {
        Signature::new(0xea7f1d08)
    }
}

impl Serialize for HkRootLevelContainerNamedVariant<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let class_meta = self._name.map(|name| (name, Signature::new(0xea7f1d08)));
        let mut serializer =
            serializer.serialize_struct("hkRootLevelContainerNamedVariant", class_meta)?;

        // For XML & Binary
        serializer.serialize_stringptr_meta_field("name", &self.name)?;
        serializer.serialize_stringptr_meta_field("className", &self.class_name)?;
        serializer.serialize_field("variant", &self.variant)?;

        // For binary
        serializer.serialize_stringptr_field("name", &self.name)?;
        serializer.serialize_stringptr_field("className", &self.class_name)?;
        serializer.end()
    }
}
