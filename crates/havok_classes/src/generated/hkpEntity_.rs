use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpEntity`
/// -         version: `3`
/// -       signature: `0xa03c774b`
/// -          size: 544(x86)/720(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpEntity<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpWorldObject<'a>,
    /// # C++ Info
    /// -          name: `material`(ctype: `struct hkpMaterial`)
    /// -        offset: 140(x86)/208(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_material: hkpMaterial,
    /// # C++ Info
    /// -          name: `limitContactImpulseUtilAndFlag`(ctype: `void*`)
    /// -        offset: 152(x86)/224(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_limitContactImpulseUtilAndFlag: Pointer,
    /// # C++ Info
    /// -          name: `damageMultiplier`(ctype: `hkReal`)
    /// -        offset: 156(x86)/232(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_damageMultiplier: f32,
    /// # C++ Info
    /// -          name: `breakableBody`(ctype: `void*`)
    /// -        offset: 160(x86)/240(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_breakableBody: Pointer,
    /// # C++ Info
    /// -          name: `solverData`(ctype: `hkUint32`)
    /// -        offset: 164(x86)/248(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_solverData: u32,
    /// # C++ Info
    /// -          name: `storageIndex`(ctype: `hkUint16`)
    /// -        offset: 168(x86)/252(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_storageIndex: u16,
    /// # C++ Info
    /// -          name: `contactPointCallbackDelay`(ctype: `hkUint16`)
    /// -        offset: 170(x86)/254(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_contactPointCallbackDelay: u16,
    /// # C++ Info
    /// -          name: `constraintsMaster`(ctype: `struct hkpEntitySmallArraySerializeOverrideType`)
    /// -        offset: 172(x86)/256(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_constraintsMaster: hkpEntitySmallArraySerializeOverrideType,
    /// # C++ Info
    /// -          name: `constraintsSlave`(ctype: `hkArray<hkpConstraintInstance*>`)
    /// -        offset: 180(x86)/272(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `NOT_OWNED|SERIALIZE_IGNORED`
    ///
    pub m_constraintsSlave: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `constraintRuntime`(ctype: `hkArray<hkUint8>`)
    /// -        offset: 192(x86)/288(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_constraintRuntime: Vec<u8>,
    /// # C++ Info
    /// -          name: `simulationIsland`(ctype: `void*`)
    /// -        offset: 204(x86)/304(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_simulationIsland: Pointer,
    /// # C++ Info
    /// -          name: `autoRemoveLevel`(ctype: `hkInt8`)
    /// -        offset: 208(x86)/312(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_autoRemoveLevel: i8,
    /// # C++ Info
    /// -          name: `numShapeKeysInContactPointProperties`(ctype: `hkUint8`)
    /// -        offset: 209(x86)/313(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_numShapeKeysInContactPointProperties: u8,
    /// # C++ Info
    /// -          name: `responseModifierFlags`(ctype: `hkUint8`)
    /// -        offset: 210(x86)/314(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_responseModifierFlags: u8,
    /// # C++ Info
    /// -          name: `uid`(ctype: `hkUint32`)
    /// -        offset: 212(x86)/316(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_uid: u32,
    /// # C++ Info
    /// -          name: `spuCollisionCallback`(ctype: `struct hkpEntitySpuCollisionCallback`)
    /// -        offset: 216(x86)/320(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_spuCollisionCallback: hkpEntitySpuCollisionCallback,
    /// # C++ Info
    /// -          name: `motion`(ctype: `struct hkpMaxSizeMotion`)
    /// -        offset: 224(x86)/336(x86_64)
    /// -     type_size: 288(x86)/320(x86_64)
    ///
    pub m_motion: hkpMaxSizeMotion,
    /// # C++ Info
    /// -          name: `contactListeners`(ctype: `struct hkpEntitySmallArraySerializeOverrideType`)
    /// -        offset: 512(x86)/656(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_contactListeners: hkpEntitySmallArraySerializeOverrideType,
    /// # C++ Info
    /// -          name: `actions`(ctype: `struct hkpEntitySmallArraySerializeOverrideType`)
    /// -        offset: 520(x86)/672(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_actions: hkpEntitySmallArraySerializeOverrideType,
    /// # C++ Info
    /// -          name: `localFrame`(ctype: `struct hkLocalFrame*`)
    /// -        offset: 528(x86)/688(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_localFrame: Pointer,
    /// # C++ Info
    /// -          name: `extendedListeners`(ctype: `struct hkpEntityExtendedListeners*`)
    /// -        offset: 532(x86)/696(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_extendedListeners: Pointer,
    /// # C++ Info
    /// -          name: `npData`(ctype: `hkUint32`)
    /// -        offset: 536(x86)/704(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_npData: u32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpEntity<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpEntity"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2688317259u32)
        }
    }
    impl<'a> _serde::Serialize for hkpEntity<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2688317259u32)));
            let mut serializer = __serializer.serialize_struct("hkpEntity", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.m_world)?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_field("collidable", &self.parent.m_collidable)?;
            serializer
                .serialize_field("multiThreadCheck", &self.parent.m_multiThreadCheck)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer
                .serialize_array_meta_field("properties", &self.parent.m_properties)?;
            serializer.skip_field("treeData", &self.parent.m_treeData)?;
            serializer.serialize_field("material", &self.m_material)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_field(
                    "limitContactImpulseUtilAndFlag",
                    &self.m_limitContactImpulseUtilAndFlag,
                )?;
            serializer.serialize_field("damageMultiplier", &self.m_damageMultiplier)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("breakableBody", &self.m_breakableBody)?;
            serializer.skip_field("solverData", &self.m_solverData)?;
            serializer.serialize_field("storageIndex", &self.m_storageIndex)?;
            serializer
                .serialize_field(
                    "contactPointCallbackDelay",
                    &self.m_contactPointCallbackDelay,
                )?;
            serializer.skip_field("constraintsMaster", &self.m_constraintsMaster)?;
            serializer
                .skip_array_meta_field("constraintsSlave", &self.m_constraintsSlave)?;
            serializer
                .skip_array_meta_field("constraintRuntime", &self.m_constraintRuntime)?;
            serializer.skip_field("simulationIsland", &self.m_simulationIsland)?;
            serializer.serialize_field("autoRemoveLevel", &self.m_autoRemoveLevel)?;
            serializer
                .serialize_field(
                    "numShapeKeysInContactPointProperties",
                    &self.m_numShapeKeysInContactPointProperties,
                )?;
            serializer
                .serialize_field(
                    "responseModifierFlags",
                    &self.m_responseModifierFlags,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("uid", &self.m_uid)?;
            serializer
                .serialize_field("spuCollisionCallback", &self.m_spuCollisionCallback)?;
            serializer.serialize_field("motion", &self.m_motion)?;
            serializer.skip_field("contactListeners", &self.m_contactListeners)?;
            serializer.skip_field("actions", &self.m_actions)?;
            serializer.serialize_field("localFrame", &self.m_localFrame)?;
            serializer.skip_field("extendedListeners", &self.m_extendedListeners)?;
            serializer.serialize_field("npData", &self.m_npData)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.serialize_array_field("properties", &self.parent.m_properties)?;
            serializer
                .serialize_array_field("constraintsSlave", &self.m_constraintsSlave)?;
            serializer
                .serialize_array_field("constraintRuntime", &self.m_constraintRuntime)?;
            serializer.end()
        }
    }
};
