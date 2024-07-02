use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbEvaluateExpressionModifierInternalExpressionData`
/// -         version: `0`
/// -       signature: `0xb8686f6b`
/// -          size:   2(x86)/  2(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbEvaluateExpressionModifierInternalExpressionData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `raisedEvent`(ctype: `hkBool`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_raisedEvent: bool,
    /// # C++ Info
    /// -          name: `wasTrueInPreviousFrame`(ctype: `hkBool`)
    /// -        offset:   1(x86)/  1(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_wasTrueInPreviousFrame: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbEvaluateExpressionModifierInternalExpressionData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbEvaluateExpressionModifierInternalExpressionData"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3093852011u32)
        }
    }
    impl __serde::Serialize for hkbEvaluateExpressionModifierInternalExpressionData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3093852011u32)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbEvaluateExpressionModifierInternalExpressionData",
                    class_meta,
                )?;
            serializer.serialize_field("raisedEvent", &self.m_raisedEvent)?;
            serializer
                .serialize_field(
                    "wasTrueInPreviousFrame",
                    &self.m_wasTrueInPreviousFrame,
                )?;
            serializer.end()
        }
    }
};
