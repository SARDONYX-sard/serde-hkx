use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpBallSocketChainDataConstraintInfo`
/// -         version: `0`
/// -       signature: `0xc9cbedf2`
/// -          size:  32(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBallSocketChainDataConstraintInfo {
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
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpBallSocketChainDataConstraintInfo {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpBallSocketChainDataConstraintInfo"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3385585138u32)
        }
    }
    impl __serde::Serialize for hkpBallSocketChainDataConstraintInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpBallSocketChainDataConstraintInfo", class_meta)?;
            serializer.serialize_field("pivotInA", &self.m_pivotInA)?;
            serializer.serialize_field("pivotInB", &self.m_pivotInB)?;
            serializer.end()
        }
    }
};
