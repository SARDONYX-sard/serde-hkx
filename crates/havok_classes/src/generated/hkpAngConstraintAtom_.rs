use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpAngConstraintAtom`
/// -         version: `0`
/// -       signature: `0x35bb3cd0`
/// -          size:   4(x86)/  4(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpAngConstraintAtom {
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
    /// -          name: `firstConstrainedAxis`(ctype: `hkUint8`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_firstConstrainedAxis: u8,
    /// # C++ Info
    /// -          name: `numConstrainedAxes`(ctype: `hkUint8`)
    /// -        offset:   3(x86)/  3(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_numConstrainedAxes: u8,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpAngConstraintAtom {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpAngConstraintAtom"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(901463248u32)
        }
    }
    impl __serde::Serialize for hkpAngConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpAngConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer
                .serialize_field("firstConstrainedAxis", &self.m_firstConstrainedAxis)?;
            serializer
                .serialize_field("numConstrainedAxes", &self.m_numConstrainedAxes)?;
            serializer.end()
        }
    }
};
