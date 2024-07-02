use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPulleyConstraintAtom`
/// -         version: `0`
/// -       signature: `0x94a08848`
/// -          size:  64(x86)/ 64(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPulleyConstraintAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintAtom,
    /// # C++ Info
    /// -          name: `fixedPivotAinWorld`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_fixedPivotAinWorld: Vector4,
    /// # C++ Info
    /// -          name: `fixedPivotBinWorld`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_fixedPivotBinWorld: Vector4,
    /// # C++ Info
    /// -          name: `ropeLength`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_ropeLength: f32,
    /// # C++ Info
    /// -          name: `leverageOnBodyB`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 52(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_leverageOnBodyB: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpPulleyConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPulleyConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2493548616u32)
        }
    }
    impl __serde::Serialize for hkpPulleyConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2493548616u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpPulleyConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer
                .serialize_field("fixedPivotAinWorld", &self.m_fixedPivotAinWorld)?;
            serializer
                .serialize_field("fixedPivotBinWorld", &self.m_fixedPivotBinWorld)?;
            serializer.serialize_field("ropeLength", &self.m_ropeLength)?;
            serializer.serialize_field("leverageOnBodyB", &self.m_leverageOnBodyB)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
