use crate::ser::Serialize;
use havok_types::{Pointer, Signature, StringPtr, Transform};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkReferencedObject {
    pub name: Option<Pointer>,

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

impl crate::HavokClass for HkReferencedObject {}
impl Serialize for HkReferencedObject {
    fn serialize<S: crate::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use crate::ser::SerializeStruct;

        let class_meta = self.name.map(|name| (name, Signature::new(0xea7f1d08)));
        let mut serializer = serializer.serialize_struct("hkReferencedObject", class_meta)?;

        serializer.skip_field("referenceCount")?;
        serializer.skip_field("memSizeAndFlags")?;
        serializer.end()
    }
}

/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xea7f1d08`
/// -   version: 0
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpShapeInfo<'a> {
    pub parent: HkReferencedObject,

    pub name: Option<Pointer>,

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

    pub v: Vec<HkReferencedObject>,
}

bitflags::bitflags! {
    /// Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++).
    ///
    /// # On XML
    /// When all bits are 0, "0" is inserted.
    /// (Even if `FLAGS_NONE = 0` and 0 is replaced by `FLAGS_NONE`, "0" will appear when reconverting xml -> hkx -> xml.)
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct FlagValues: u16 {
        /// Flags is empty: 0
        const FLAGS_NONE = 0;
        /// Force 8-byte align: 1 << 7
        const ALIGN_8 = 128;
        /// Forced 16-byte align: 1 << 8
        const ALIGN_16 = 256;
        /// Not owned by class: 1 << 9
        const NOT_OWNED = 512;
        /// Skip serializing: 1 << 10
        const SERIALIZE_IGNORED = 1024;
    }
}

impl Serialize for FlagValues {
    fn serialize<S: crate::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use crate::ser::SerializeFlags;

        let mut sv = serializer.serialize_flags()?;
        if self.is_empty() {
            sv.serialize_empty_bit()?;
            return sv.end();
        };

        for flag in self.iter() {
            match flag {
                FlagValues::FLAGS_NONE => sv.serialize_field("FLAGS_NONE", &FlagValues::FLAGS_NONE),
                FlagValues::ALIGN_8 => sv.serialize_field("ALIGN_8", &FlagValues::ALIGN_8),
                FlagValues::ALIGN_16 => sv.serialize_field("ALIGN_16", &FlagValues::ALIGN_16),
                FlagValues::NOT_OWNED => sv.serialize_field("NOT_OWNED", &FlagValues::NOT_OWNED),
                FlagValues::SERIALIZE_IGNORED => {
                    sv.serialize_field("SERIALIZE_IGNORED", &FlagValues::SERIALIZE_IGNORED)
                }
                remain => sv.serialize_field(&remain.bits().to_string(), &remain.bits()),
            }?
        }
        sv.end()
    }
}

impl crate::HavokClass for HkpShapeInfo<'_> {}
impl Serialize for HkpShapeInfo<'_> {
    fn serialize<S: crate::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use crate::ser::SerializeStruct;

        let class_meta = self.name.map(|name| (name, Signature::new(0xea7f1d08)));
        let mut serializer = serializer.serialize_struct("hkpShapeInfo", class_meta)?;

        // Serialize fields of parent (flatten)
        serializer.skip_field("memSizeAndFlags")?;
        serializer.skip_field("referenceCount")?;

        serializer.serialize_field("shape", &self.shape)?;
        serializer.serialize_field("isHierarchicalCompound", &self.is_hierarchical_compound)?;
        serializer.serialize_field("hkdShapesCollected", &self.hkd_shapes_collected)?;
        serializer.serialize_array_field("childShapeNames", &self.child_shape_names)?;
        serializer.serialize_array_field("childTransforms", &self.child_transforms)?;
        serializer.serialize_field("transform", &self.transform)?;

        serializer.serialize_array_field("class_vector", &self.v)?;

        serializer.serialize_field("flags_none_is_0", &FlagValues::FLAGS_NONE)?;
        serializer.serialize_field(
            "no_display_0_flag_when_with_other",
            &(FlagValues::FLAGS_NONE | FlagValues::NOT_OWNED),
        )?;
        serializer.serialize_field(
            "flags_non_remain",
            &(FlagValues::NOT_OWNED | FlagValues::ALIGN_16),
        )?;
        serializer.serialize_field(
            "flags_with_remain",
            &(FlagValues::FLAGS_NONE
                | FlagValues::NOT_OWNED
                | FlagValues::SERIALIZE_IGNORED
                | FlagValues::from_bits_retain(1)),
        )?;

        serializer.end()
    }
}

pub enum Classes<'a> {
    HkpShapeInfo(HkpShapeInfo<'a>),
    HkReferencedObject(HkReferencedObject),
}

impl crate::HavokClass for Classes<'_> {}
impl<'a> Serialize for Classes<'a> {
    fn serialize<S: crate::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Classes::HkpShapeInfo(class) => class.serialize(serializer),
            Classes::HkReferencedObject(class) => class.serialize(serializer),
        }
    }
}
