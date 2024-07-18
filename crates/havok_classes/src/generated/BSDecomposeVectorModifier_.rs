use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `BSDecomposeVectorModifier`
/// - version: `0`
/// - signature: `0x31f6b8b6`
/// - size: ` 80`(x86)/`112`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSDecomposeVectorModifier<'a> {
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
    /// - name: `vector`(ctype: `hkVector4`)
    /// - offset: ` 48`(x86)/` 80`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_vector: Vector4,
    /// # C++ Info
    /// - name: `x`(ctype: `hkReal`)
    /// - offset: ` 64`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_x: f32,
    /// # C++ Info
    /// - name: `y`(ctype: `hkReal`)
    /// - offset: ` 68`(x86)/`100`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_y: f32,
    /// # C++ Info
    /// - name: `z`(ctype: `hkReal`)
    /// - offset: ` 72`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_z: f32,
    /// # C++ Info
    /// - name: `w`(ctype: `hkReal`)
    /// - offset: ` 76`(x86)/`108`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_w: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSDecomposeVectorModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSDecomposeVectorModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x31f6b8b6)
        }
    }
    impl<'a> _serde::Serialize for BSDecomposeVectorModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x31f6b8b6)));
            let mut serializer = __serializer
                .serialize_struct("BSDecomposeVectorModifier", class_meta)?;
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
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("vector", &self.m_vector)?;
            serializer.serialize_field("x", &self.m_x)?;
            serializer.serialize_field("y", &self.m_y)?;
            serializer.serialize_field("z", &self.m_z)?;
            serializer.serialize_field("w", &self.m_w)?;
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
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_vector,
    m_x,
    m_y,
    m_z,
    m_w,
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
            "vector" => Ok(__Field::m_vector),
            "x" => Ok(__Field::m_x),
            "y" => Ok(__Field::m_y),
            "z" => Ok(__Field::m_z),
            "w" => Ok(__Field::m_w),
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
pub(super) struct __BSDecomposeVectorModifierVisitor<'de> {
    marker: core::marker::PhantomData<BSDecomposeVectorModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSDecomposeVectorModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSDecomposeVectorModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<BSDecomposeVectorModifier<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __BSDecomposeVectorModifierVisitor<'de> {
    type Value = BSDecomposeVectorModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct BSDecomposeVectorModifier")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __A::parent_value(&mut __map)?;
        let mut m_vector: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_x: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_y: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_z: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_w: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_vector) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("vector"),
                        );
                    }
                    __A::pad(&mut __map, 4usize, 0usize)?;
                    m_vector = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_x) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("x"),
                        );
                    }
                    m_x = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_y) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("y"),
                        );
                    }
                    m_y = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_z) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("z"),
                        );
                    }
                    m_z = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_w) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("w"),
                        );
                    }
                    m_w = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                _ => {}
            }
        }
        let m_vector = match m_vector {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vector"),
                );
            }
        };
        let m_x = match m_x {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("x"),
                );
            }
        };
        let m_y = match m_y {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("y"),
                );
            }
        };
        let m_z = match m_z {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("z"),
                );
            }
        };
        let m_w = match m_w {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("w"),
                );
            }
        };
        _serde::__private::Ok(BSDecomposeVectorModifier {
            __ptr,
            parent,
            m_vector,
            m_x,
            m_y,
            m_z,
            m_w,
        })
    }
    #[allow(clippy::manual_unwrap_or_default)]
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_vector: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_x: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_y: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_z: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_w: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..5usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_vector => {
                        if _serde::__private::Option::is_some(&m_vector) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("vector"),
                            );
                        }
                        m_vector = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_x => {
                        if _serde::__private::Option::is_some(&m_x) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("x"),
                            );
                        }
                        m_x = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_y => {
                        if _serde::__private::Option::is_some(&m_y) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("y"),
                            );
                        }
                        m_y = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_z => {
                        if _serde::__private::Option::is_some(&m_z) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("z"),
                            );
                        }
                        m_z = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_w => {
                        if _serde::__private::Option::is_some(&m_w) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("w"),
                            );
                        }
                        m_w = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        let m_vector = match m_vector {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vector"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_x = match m_x {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("x"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_y = match m_y {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("y"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_z = match m_z {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("z"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_w = match m_w {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("w"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(BSDecomposeVectorModifier {
            __ptr,
            parent,
            m_vector,
            m_x,
            m_y,
            m_z,
            m_w,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSDecomposeVectorModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["vector", "x", "y", "z", "w"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSDecomposeVectorModifier",
                FIELDS,
                __BSDecomposeVectorModifierVisitor {
                    marker: _serde::__private::PhantomData::<BSDecomposeVectorModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
