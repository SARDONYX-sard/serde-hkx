use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkGizmoAttribute`
/// -         version: `0`
/// -       signature: `0x23aadfb6`
/// -          size:  12(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkGizmoAttribute<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `visible`(ctype: `hkBool`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_visible: bool,
    /// # C++ Info
    /// -          name: `label`(ctype: `char*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_label: CString<'a>,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum GizmoType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: GizmoType,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkGizmoAttribute<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkGizmoAttribute"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(598400950u32)
        }
    }
    impl<'a> __serde::Serialize for hkGizmoAttribute<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkGizmoAttribute", class_meta)?;
            serializer.serialize_field("visible", &self.m_visible)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_cstring_meta_field("label", &self.m_label)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_cstring_field("label", &self.m_label)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum GizmoType {
    #[default]
    POINT = 0isize,
    SPHERE = 1isize,
    PLANE = 2isize,
    ARROW = 3isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for GizmoType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::POINT => __serializer.serialize_field("POINT", &0u64),
                Self::SPHERE => __serializer.serialize_field("SPHERE", &1u64),
                Self::PLANE => __serializer.serialize_field("PLANE", &2u64),
                Self::ARROW => __serializer.serialize_field("ARROW", &3u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum GizmoType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
