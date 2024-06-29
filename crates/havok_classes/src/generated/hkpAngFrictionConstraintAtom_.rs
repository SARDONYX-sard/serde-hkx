use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpAngFrictionConstraintAtom`
/// -         version: `0`
/// -       signature: `0xf313aa80`
/// -          size:  12(x86)/ 12(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpAngFrictionConstraintAtom {
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
    /// -          name: `firstFrictionAxis`(ctype: `hkUint8`)
    /// -        offset:   3(x86)/  3(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_firstFrictionAxis: u8,
    /// # C++ Info
    /// -          name: `numFrictionAxes`(ctype: `hkUint8`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_numFrictionAxes: u8,
    /// # C++ Info
    /// -          name: `maxFrictionTorque`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxFrictionTorque: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpAngFrictionConstraintAtom {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpAngFrictionConstraintAtom"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4078152320u32)
        }
    }
    impl __serde::Serialize for hkpAngFrictionConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpAngFrictionConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("isEnabled", &self.m_isEnabled)?;
            serializer.serialize_field("firstFrictionAxis", &self.m_firstFrictionAxis)?;
            serializer.serialize_field("numFrictionAxes", &self.m_numFrictionAxes)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("maxFrictionTorque", &self.m_maxFrictionTorque)?;
            serializer.end()
        }
    }
};
