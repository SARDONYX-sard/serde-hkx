use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpFirstPersonGun`
/// -         version: `0`
/// -       signature: `0x852ab70b`
/// -          size:  32(x86)/ 56(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpFirstPersonGun<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum unknown`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_type: u8,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `keyboardKey`(ctype: `enum KeyboardKey`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_keyboardKey: KeyboardKey,
    /// # C++ Info
    /// -          name: `listeners`(ctype: `hkArray<void*>`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_listeners: Vec<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpFirstPersonGun<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpFirstPersonGun"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2234169099u32)
        }
    }
    impl<'a> __serde::Serialize for hkpFirstPersonGun<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpFirstPersonGun", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("type", &self.m_type)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_field("keyboardKey", &self.m_keyboardKey)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.skip_array_meta_field("listeners", &self.m_listeners)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.serialize_array_field("listeners", &self.m_listeners)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_UINT8`
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
pub enum Type {
    #[default]
    WEAPON_TYPE_INVALID = 0isize,
    WEAPON_TYPE_BALLGUN = 1isize,
    WEAPON_TYPE_GRENADEGUN = 2isize,
    WEAPON_TYPE_GRAVITYGUN = 3isize,
    WEAPON_TYPE_MOUNTEDBALLGUN = 4isize,
    WEAPON_TYPE_TWEAKERGUN = 5isize,
    WEAPON_TYPE_MISSILEGUN = 6isize,
    WEAPON_TYPE_RAYCASTGUN = 7isize,
    WEAPON_TYPE_SPHEREGUN = 8isize,
    WEAPON_TYPE_STICKYGUN = 9isize,
    WEAPON_TYPE_NUM_TYPES = 10isize,
}
///- size(C++): `TYPE_UINT8`
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
pub enum KeyboardKey {
    #[default]
    KEY_F1 = 112isize,
    KEY_F2 = 113isize,
    KEY_F3 = 114isize,
    KEY_F4 = 115isize,
    KEY_F5 = 116isize,
    KEY_F6 = 117isize,
    KEY_F7 = 118isize,
    KEY_F8 = 119isize,
    KEY_F9 = 120isize,
    KEY_F10 = 121isize,
    KEY_F11 = 122isize,
    KEY_F12 = 123isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Type {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::WEAPON_TYPE_INVALID => {
                    __serializer.serialize_field("WEAPON_TYPE_INVALID", &0u64)
                }
                Self::WEAPON_TYPE_BALLGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_BALLGUN", &1u64)
                }
                Self::WEAPON_TYPE_GRENADEGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_GRENADEGUN", &2u64)
                }
                Self::WEAPON_TYPE_GRAVITYGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_GRAVITYGUN", &3u64)
                }
                Self::WEAPON_TYPE_MOUNTEDBALLGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_MOUNTEDBALLGUN", &4u64)
                }
                Self::WEAPON_TYPE_TWEAKERGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_TWEAKERGUN", &5u64)
                }
                Self::WEAPON_TYPE_MISSILEGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_MISSILEGUN", &6u64)
                }
                Self::WEAPON_TYPE_RAYCASTGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_RAYCASTGUN", &7u64)
                }
                Self::WEAPON_TYPE_SPHEREGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_SPHEREGUN", &8u64)
                }
                Self::WEAPON_TYPE_STICKYGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_STICKYGUN", &9u64)
                }
                Self::WEAPON_TYPE_NUM_TYPES => {
                    __serializer.serialize_field("WEAPON_TYPE_NUM_TYPES", &10u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_u8().ok_or(S::Error::custom("Failed enum Type to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for KeyboardKey {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::KEY_F1 => __serializer.serialize_field("KEY_F1", &112u64),
                Self::KEY_F2 => __serializer.serialize_field("KEY_F2", &113u64),
                Self::KEY_F3 => __serializer.serialize_field("KEY_F3", &114u64),
                Self::KEY_F4 => __serializer.serialize_field("KEY_F4", &115u64),
                Self::KEY_F5 => __serializer.serialize_field("KEY_F5", &116u64),
                Self::KEY_F6 => __serializer.serialize_field("KEY_F6", &117u64),
                Self::KEY_F7 => __serializer.serialize_field("KEY_F7", &118u64),
                Self::KEY_F8 => __serializer.serialize_field("KEY_F8", &119u64),
                Self::KEY_F9 => __serializer.serialize_field("KEY_F9", &120u64),
                Self::KEY_F10 => __serializer.serialize_field("KEY_F10", &121u64),
                Self::KEY_F11 => __serializer.serialize_field("KEY_F11", &122u64),
                Self::KEY_F12 => __serializer.serialize_field("KEY_F12", &123u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum KeyboardKey to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
