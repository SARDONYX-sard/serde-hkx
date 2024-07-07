use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaKeyFrameHierarchyUtility`
/// -         version: `0`
/// -       signature: `0x7bd5c66f`
/// -          size:   1(x86)/  1(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaKeyFrameHierarchyUtility {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaKeyFrameHierarchyUtility {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaKeyFrameHierarchyUtility"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7bd5c66f)
        }
    }
    impl _serde::Serialize for hkaKeyFrameHierarchyUtility {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x7bd5c66f)));
            let mut serializer = __serializer
                .serialize_struct("hkaKeyFrameHierarchyUtility", class_meta)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.end()
        }
    }
};
