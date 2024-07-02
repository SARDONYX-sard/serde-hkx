use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkSweptTransform`
/// -         version: `0`
/// -       signature: `0xb4e5770`
/// -          size:  80(x86)/ 80(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkSweptTransform {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `centerOfMass0`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_centerOfMass0: Vector4,
    /// # C++ Info
    /// -          name: `centerOfMass1`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_centerOfMass1: Vector4,
    /// # C++ Info
    /// -          name: `rotation0`(ctype: `hkQuaternion`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rotation0: Quaternion,
    /// # C++ Info
    /// -          name: `rotation1`(ctype: `hkQuaternion`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rotation1: Quaternion,
    /// # C++ Info
    /// -          name: `centerOfMassLocal`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_centerOfMassLocal: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkSweptTransform {
        #[inline]
        fn name(&self) -> &'static str {
            "hkSweptTransform"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(189683568u32)
        }
    }
    impl _serde::Serialize for hkSweptTransform {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(189683568u32)));
            let mut serializer = __serializer
                .serialize_struct("hkSweptTransform", class_meta)?;
            serializer.serialize_field("centerOfMass0", &self.m_centerOfMass0)?;
            serializer.serialize_field("centerOfMass1", &self.m_centerOfMass1)?;
            serializer.serialize_field("rotation0", &self.m_rotation0)?;
            serializer.serialize_field("rotation1", &self.m_rotation1)?;
            serializer.serialize_field("centerOfMassLocal", &self.m_centerOfMassLocal)?;
            serializer.end()
        }
    }
};
