use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPoweredChainDataConstraintInfo`
/// -         version: `0`
/// -       signature: `0xf88aee25`
/// -          size:  80(x86)/ 96(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPoweredChainDataConstraintInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `pivotInA`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_pivotInA: Vector4,
    /// # C++ Info
    /// -          name: `pivotInB`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_pivotInB: Vector4,
    /// # C++ Info
    /// -          name: `aTc`(ctype: `hkQuaternion`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_aTc: Quaternion,
    /// # C++ Info
    /// -          name: `bTc`(ctype: `hkQuaternion`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_bTc: Quaternion,
    /// # C++ Info
    /// -          name: `motors`(ctype: `struct hkpConstraintMotor*`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 24(x86_64)
    ///
    pub m_motors: [Pointer; 3usize],
    /// # C++ Info
    /// -          name: `switchBodies`(ctype: `hkBool`)
    /// -        offset:  76(x86)/ 88(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_switchBodies: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpPoweredChainDataConstraintInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPoweredChainDataConstraintInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf88aee25)
        }
    }
    impl _serde::Serialize for hkpPoweredChainDataConstraintInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf88aee25)));
            let mut serializer = __serializer
                .serialize_struct("hkpPoweredChainDataConstraintInfo", class_meta)?;
            serializer.serialize_field("pivotInA", &self.m_pivotInA)?;
            serializer.serialize_field("pivotInB", &self.m_pivotInB)?;
            serializer.serialize_field("aTc", &self.m_aTc)?;
            serializer.serialize_field("bTc", &self.m_bTc)?;
            serializer.serialize_field("motors", &self.m_motors.as_slice())?;
            serializer.serialize_field("switchBodies", &self.m_switchBodies)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
