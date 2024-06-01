use super::*;

/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xea7f1d08`
/// -   version: 0
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpShapeInfo<'a> {
    pub parent: HkReferencedObject,

    pub _name: Option<Pointer>,

    /// # C++ Class Fields Info
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub shape: Pointer,
    /// # C++ Class Fields Info
    /// -   name:`"isHierarchicalCompound"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub is_hierarchical_compound: bool,
    /// # C++ Class Fields Info
    /// -   name:`"hkdShapesCollected"`
    /// -   type: `hkBool`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    pub hkd_shapes_collected: bool,
    /// # C++ Class Fields Info
    /// -   name:`"childShapeNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub child_shape_names: Vec<StringPtr<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"childTransforms"`
    /// -   type: `hkArray<hkTransform>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub child_transforms: Vec<Transform>,
    /// # C++ Class Fields Info
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub transform: Transform,
}

impl HavokClass for HkpShapeInfo<'_> {
    fn name(&self) -> &'static CStr {
        c"hkpShapeInfo"
    }

    fn signature(&self) -> Signature {
        Signature::new(0xea7f1d08)
    }
}

impl Serialize for HkpShapeInfo<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let class_meta = self._name.map(|name| (name, self.signature()));
        let mut serializer = serializer.serialize_struct("hkpShapeInfo", class_meta)?;

        // Serialize fields of parent (flatten)
        serializer.skip_field("referenceCount", &self.parent.reference_count)?;
        serializer.skip_field("memSizeAndFlags", &self.parent.mem_size_and_flags)?;

        // For XML & binary
        serializer.serialize_field("shape", &self.shape)?;
        serializer.serialize_field("isHierarchicalCompound", &self.is_hierarchical_compound)?;
        serializer.serialize_field("hkdShapesCollected", &self.hkd_shapes_collected)?;
        serializer.serialize_array_meta_field("childShapeNames", &self.child_shape_names)?;
        serializer.serialize_array_meta_field("childTransforms", &self.child_transforms)?;
        serializer.serialize_field("transform", &self.transform)?;

        // For binary
        serializer.serialize_array_field("childShapeNames", &self.child_shape_names)?;
        serializer.serialize_array_field("childTransforms", &self.child_transforms)?;

        serializer.end()
    }
}
