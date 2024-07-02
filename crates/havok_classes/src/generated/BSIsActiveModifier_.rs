use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSIsActiveModifier`
/// -         version: `1`
/// -       signature: `0xb0fde45a`
/// -          size:  56(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSIsActiveModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `bIsActive0`(ctype: `hkBool`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bIsActive0: bool,
    /// # C++ Info
    /// -          name: `bInvertActive0`(ctype: `hkBool`)
    /// -        offset:  45(x86)/ 81(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bInvertActive0: bool,
    /// # C++ Info
    /// -          name: `bIsActive1`(ctype: `hkBool`)
    /// -        offset:  46(x86)/ 82(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bIsActive1: bool,
    /// # C++ Info
    /// -          name: `bInvertActive1`(ctype: `hkBool`)
    /// -        offset:  47(x86)/ 83(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bInvertActive1: bool,
    /// # C++ Info
    /// -          name: `bIsActive2`(ctype: `hkBool`)
    /// -        offset:  48(x86)/ 84(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bIsActive2: bool,
    /// # C++ Info
    /// -          name: `bInvertActive2`(ctype: `hkBool`)
    /// -        offset:  49(x86)/ 85(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bInvertActive2: bool,
    /// # C++ Info
    /// -          name: `bIsActive3`(ctype: `hkBool`)
    /// -        offset:  50(x86)/ 86(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bIsActive3: bool,
    /// # C++ Info
    /// -          name: `bInvertActive3`(ctype: `hkBool`)
    /// -        offset:  51(x86)/ 87(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bInvertActive3: bool,
    /// # C++ Info
    /// -          name: `bIsActive4`(ctype: `hkBool`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bIsActive4: bool,
    /// # C++ Info
    /// -          name: `bInvertActive4`(ctype: `hkBool`)
    /// -        offset:  53(x86)/ 89(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bInvertActive4: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSIsActiveModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSIsActiveModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2969429082u32)
        }
    }
    impl<'a> _serde::Serialize for BSIsActiveModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2969429082u32)));
            let mut serializer = __serializer
                .serialize_struct("BSIsActiveModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("bIsActive0", &self.m_bIsActive0)?;
            serializer.serialize_field("bInvertActive0", &self.m_bInvertActive0)?;
            serializer.serialize_field("bIsActive1", &self.m_bIsActive1)?;
            serializer.serialize_field("bInvertActive1", &self.m_bInvertActive1)?;
            serializer.serialize_field("bIsActive2", &self.m_bIsActive2)?;
            serializer.serialize_field("bInvertActive2", &self.m_bInvertActive2)?;
            serializer.serialize_field("bIsActive3", &self.m_bIsActive3)?;
            serializer.serialize_field("bInvertActive3", &self.m_bInvertActive3)?;
            serializer.serialize_field("bIsActive4", &self.m_bIsActive4)?;
            serializer.serialize_field("bInvertActive4", &self.m_bInvertActive4)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
