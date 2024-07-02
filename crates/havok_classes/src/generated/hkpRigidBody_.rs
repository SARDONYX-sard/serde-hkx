use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpRigidBody`
/// -         version: `0`
/// -       signature: `0x75f8d805`
/// -          size: 544(x86)/720(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpRigidBody<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpEntity<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpRigidBody<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpRigidBody"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1979242501u32)
        }
    }
    impl<'a> _serde::Serialize for hkpRigidBody<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1979242501u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpRigidBody", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.parent.m_world)?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.serialize_field("collidable", &self.parent.parent.m_collidable)?;
            serializer
                .serialize_field(
                    "multiThreadCheck",
                    &self.parent.parent.m_multiThreadCheck,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_array_meta_field(
                    "properties",
                    &self.parent.parent.m_properties,
                )?;
            serializer.skip_field("treeData", &self.parent.parent.m_treeData)?;
            serializer.serialize_field("material", &self.parent.m_material)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_field(
                    "limitContactImpulseUtilAndFlag",
                    &self.parent.m_limitContactImpulseUtilAndFlag,
                )?;
            serializer
                .serialize_field("damageMultiplier", &self.parent.m_damageMultiplier)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("breakableBody", &self.parent.m_breakableBody)?;
            serializer.skip_field("solverData", &self.parent.m_solverData)?;
            serializer.serialize_field("storageIndex", &self.parent.m_storageIndex)?;
            serializer
                .serialize_field(
                    "contactPointCallbackDelay",
                    &self.parent.m_contactPointCallbackDelay,
                )?;
            serializer
                .skip_field("constraintsMaster", &self.parent.m_constraintsMaster)?;
            serializer
                .skip_array_meta_field(
                    "constraintsSlave",
                    &self.parent.m_constraintsSlave,
                )?;
            serializer
                .skip_array_meta_field(
                    "constraintRuntime",
                    &self.parent.m_constraintRuntime,
                )?;
            serializer.skip_field("simulationIsland", &self.parent.m_simulationIsland)?;
            serializer
                .serialize_field("autoRemoveLevel", &self.parent.m_autoRemoveLevel)?;
            serializer
                .serialize_field(
                    "numShapeKeysInContactPointProperties",
                    &self.parent.m_numShapeKeysInContactPointProperties,
                )?;
            serializer
                .serialize_field(
                    "responseModifierFlags",
                    &self.parent.m_responseModifierFlags,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("uid", &self.parent.m_uid)?;
            serializer
                .serialize_field(
                    "spuCollisionCallback",
                    &self.parent.m_spuCollisionCallback,
                )?;
            serializer.serialize_field("motion", &self.parent.m_motion)?;
            serializer.skip_field("contactListeners", &self.parent.m_contactListeners)?;
            serializer.skip_field("actions", &self.parent.m_actions)?;
            serializer.serialize_field("localFrame", &self.parent.m_localFrame)?;
            serializer
                .skip_field("extendedListeners", &self.parent.m_extendedListeners)?;
            serializer.serialize_field("npData", &self.parent.m_npData)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_array_field("properties", &self.parent.parent.m_properties)?;
            serializer
                .serialize_array_field(
                    "constraintsSlave",
                    &self.parent.m_constraintsSlave,
                )?;
            serializer
                .serialize_array_field(
                    "constraintRuntime",
                    &self.parent.m_constraintRuntime,
                )?;
            serializer.end()
        }
    }
};
