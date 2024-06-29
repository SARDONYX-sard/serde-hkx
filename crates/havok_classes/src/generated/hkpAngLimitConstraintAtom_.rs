use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpAngLimitConstraintAtom`
/// -         version: `0`
/// -       signature: `0x9be0d9d`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpAngLimitConstraintAtom {
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
    /// -          name: `isEnabled`(ctype: `hkUint8`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isEnabled: u8,
    /// # C++ Info
    /// -          name: `limitAxis`(ctype: `hkUint8`)
    /// -        offset:   3(x86)/  3(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_limitAxis: u8,
    /// # C++ Info
    /// -          name: `minAngle`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minAngle: f32,
    /// # C++ Info
    /// -          name: `maxAngle`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxAngle: f32,
    /// # C++ Info
    /// -          name: `angularLimitsTauFactor`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_angularLimitsTauFactor: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpAngLimitConstraintAtom {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpAngLimitConstraintAtom"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(163450269u32)
        }
    }
    impl __serde::Serialize for hkpAngLimitConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpAngLimitConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("isEnabled", &self.m_isEnabled)?;
            serializer.serialize_field("limitAxis", &self.m_limitAxis)?;
            serializer.serialize_field("minAngle", &self.m_minAngle)?;
            serializer.serialize_field("maxAngle", &self.m_maxAngle)?;
            serializer
                .serialize_field(
                    "angularLimitsTauFactor",
                    &self.m_angularLimitsTauFactor,
                )?;
            serializer.end()
        }
    }
};
