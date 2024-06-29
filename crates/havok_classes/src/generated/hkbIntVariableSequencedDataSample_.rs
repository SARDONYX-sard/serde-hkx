use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbIntVariableSequencedDataSample`
/// -         version: `0`
/// -       signature: `0xbe7ac63c`
/// -          size:   8(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbIntVariableSequencedDataSample {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `time`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_time: f32,
    /// # C++ Info
    /// -          name: `value`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_value: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbIntVariableSequencedDataSample {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbIntVariableSequencedDataSample"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3195717180u32)
        }
    }
    impl __serde::Serialize for hkbIntVariableSequencedDataSample {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbIntVariableSequencedDataSample", class_meta)?;
            serializer.serialize_field("time", &self.m_time)?;
            serializer.serialize_field("value", &self.m_value)?;
            serializer.end()
        }
    }
};
