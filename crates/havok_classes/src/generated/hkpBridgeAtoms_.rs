use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpBridgeAtoms`
/// -         version: `0`
/// -       signature: `0xde152a4d`
/// -          size:  12(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBridgeAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `bridgeAtom`(ctype: `struct hkpBridgeConstraintAtom`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 24(x86_64)
    ///
    pub m_bridgeAtom: hkpBridgeConstraintAtom,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpBridgeAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpBridgeAtoms"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3725929037u32)
        }
    }
    impl __serde::Serialize for hkpBridgeAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3725929037u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpBridgeAtoms", class_meta)?;
            serializer.serialize_field("bridgeAtom", &self.m_bridgeAtom)?;
            serializer.end()
        }
    }
};
