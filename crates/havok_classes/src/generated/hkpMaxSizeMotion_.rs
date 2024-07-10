use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMaxSizeMotion`
/// -         version: `0`
/// -       signature: `0x64abf85c`
/// -          size: 288(x86)/320(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMaxSizeMotion {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpKeyframedRigidMotion,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpMaxSizeMotion {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMaxSizeMotion"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x64abf85c)
        }
    }
    impl _serde::Serialize for hkpMaxSizeMotion {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x64abf85c)));
            let mut serializer = __serializer
                .serialize_struct("hkpMaxSizeMotion", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.parent.parent.m_type)?;
            serializer
                .serialize_field(
                    "deactivationIntegrateCounter",
                    &self.parent.parent.m_deactivationIntegrateCounter,
                )?;
            serializer
                .serialize_field(
                    "deactivationNumInactiveFrames",
                    &self.parent.parent.m_deactivationNumInactiveFrames.as_slice(),
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 10usize].as_slice())?;
            serializer
                .serialize_field("motionState", &self.parent.parent.m_motionState)?;
            serializer
                .serialize_field(
                    "inertiaAndMassInv",
                    &self.parent.parent.m_inertiaAndMassInv,
                )?;
            serializer
                .serialize_field(
                    "linearVelocity",
                    &self.parent.parent.m_linearVelocity,
                )?;
            serializer
                .serialize_field(
                    "angularVelocity",
                    &self.parent.parent.m_angularVelocity,
                )?;
            serializer
                .serialize_field(
                    "deactivationRefPosition",
                    &self.parent.parent.m_deactivationRefPosition.as_slice(),
                )?;
            serializer
                .serialize_field(
                    "deactivationRefOrientation",
                    &self.parent.parent.m_deactivationRefOrientation.as_slice(),
                )?;
            serializer
                .serialize_field("savedMotion", &self.parent.parent.m_savedMotion)?;
            serializer
                .serialize_field(
                    "savedQualityTypeIndex",
                    &self.parent.parent.m_savedQualityTypeIndex,
                )?;
            serializer
                .serialize_field("gravityFactor", &self.parent.parent.m_gravityFactor)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    __ignore,
}
struct __FieldVisitor;
impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
    type Value = __Field;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "field identifier")
    }
    /// Intended for use in XML.
    #[allow(clippy::match_single_binding)]
    #[allow(clippy::reversed_empty_ranges)]
    #[allow(clippy::single_match)]
    fn visit_key<__E>(self, __value: &str) -> core::result::Result<Self::Value, __E>
    where
        __E: _serde::de::Error,
    {
        match __value {
            _ => Ok(__Field::__ignore),
        }
    }
}
impl<'de> _serde::Deserialize<'de> for __Field {
    #[inline]
    fn deserialize<__D>(__deserializer: __D) -> core::result::Result<Self, __D::Error>
    where
        __D: _serde::Deserializer<'de>,
    {
        _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
    }
}
pub(super) struct __hkpMaxSizeMotionVisitor<'de> {
    marker: core::marker::PhantomData<hkpMaxSizeMotion>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpMaxSizeMotionVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpMaxSizeMotion, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpMaxSizeMotion>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpMaxSizeMotionVisitor<'de> {
    type Value = hkpMaxSizeMotion;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpMaxSizeMotion")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        for i in 0..0usize {
            match i {
                _ => {}
            }
        }
        __A::pad(&mut __map, 288usize, 320usize)?;
        _serde::__private::Ok(hkpMaxSizeMotion {
            __ptr: __A::class_ptr(&mut __map),
            parent,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpKeyframedRigidMotionVisitor::visit_as_parent(&mut __map)?;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                _ => {}
            }
        }
        _serde::__private::Ok(hkpMaxSizeMotion {
            __ptr: __A::class_ptr(&mut __map),
            parent,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpMaxSizeMotion {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpMaxSizeMotion",
                FIELDS,
                __hkpMaxSizeMotionVisitor {
                    marker: _serde::__private::PhantomData::<hkpMaxSizeMotion>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
