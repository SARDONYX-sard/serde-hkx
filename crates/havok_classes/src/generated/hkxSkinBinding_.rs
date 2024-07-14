use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxSkinBinding`
/// -         version: `2`
/// -       signature: `0x5a93f338`
/// -          size: 112(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxSkinBinding<'a> {
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
    /// -          name: `mesh`(ctype: `struct hkxMesh*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_mesh: Pointer,
    /// # C++ Info
    /// -          name: `nodeNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_nodeNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `bindPose`(ctype: `hkArray<hkMatrix4>`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_bindPose: Vec<Matrix4>,
    /// # C++ Info
    /// -          name: `initSkinTransform`(ctype: `hkMatrix4`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_initSkinTransform: Matrix4,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkxSkinBinding<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxSkinBinding"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5a93f338)
        }
    }
    impl<'a> _serde::Serialize for hkxSkinBinding<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x5a93f338)));
            let mut serializer = __serializer
                .serialize_struct("hkxSkinBinding", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("mesh", &self.m_mesh)?;
            serializer.serialize_array_meta_field("nodeNames", &self.m_nodeNames)?;
            serializer.serialize_array_meta_field("bindPose", &self.m_bindPose)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("initSkinTransform", &self.m_initSkinTransform)?;
            serializer.serialize_array_field("nodeNames", &self.m_nodeNames)?;
            serializer.serialize_array_field("bindPose", &self.m_bindPose)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_mesh,
    m_nodeNames,
    m_bindPose,
    m_initSkinTransform,
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
            "mesh" => Ok(__Field::m_mesh),
            "nodeNames" => Ok(__Field::m_nodeNames),
            "bindPose" => Ok(__Field::m_bindPose),
            "initSkinTransform" => Ok(__Field::m_initSkinTransform),
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
pub(super) struct __hkxSkinBindingVisitor<'de> {
    marker: core::marker::PhantomData<hkxSkinBinding<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkxSkinBindingVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkxSkinBinding<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkxSkinBinding<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkxSkinBindingVisitor<'de> {
    type Value = hkxSkinBinding<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkxSkinBinding")
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
        let mut m_mesh: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_nodeNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_bindPose: _serde::__private::Option<Vec<Matrix4>> = _serde::__private::None;
        let mut m_initSkinTransform: _serde::__private::Option<Matrix4> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_mesh) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("mesh"),
                        );
                    }
                    m_mesh = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_nodeNames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nodeNames",
                            ),
                        );
                    }
                    m_nodeNames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_bindPose) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bindPose",
                            ),
                        );
                    }
                    m_bindPose = _serde::__private::Some(
                        match __A::next_value::<Vec<Matrix4>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_initSkinTransform) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "initSkinTransform",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 12usize, 8usize)?;
                    m_initSkinTransform = _serde::__private::Some(
                        match __A::next_value::<Matrix4>(&mut __map) {
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
        let m_mesh = match m_mesh {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("mesh"),
                );
            }
        };
        let m_nodeNames = match m_nodeNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nodeNames"),
                );
            }
        };
        let m_bindPose = match m_bindPose {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bindPose"),
                );
            }
        };
        let m_initSkinTransform = match m_initSkinTransform {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("initSkinTransform"),
                );
            }
        };
        _serde::__private::Ok(hkxSkinBinding {
            __ptr,
            parent,
            m_mesh,
            m_nodeNames,
            m_bindPose,
            m_initSkinTransform,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_mesh: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_nodeNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_bindPose: _serde::__private::Option<Vec<Matrix4>> = _serde::__private::None;
        let mut m_initSkinTransform: _serde::__private::Option<Matrix4> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_mesh => {
                        if _serde::__private::Option::is_some(&m_mesh) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("mesh"),
                            );
                        }
                        m_mesh = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_nodeNames => {
                        if _serde::__private::Option::is_some(&m_nodeNames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "nodeNames",
                                ),
                            );
                        }
                        m_nodeNames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_bindPose => {
                        if _serde::__private::Option::is_some(&m_bindPose) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bindPose",
                                ),
                            );
                        }
                        m_bindPose = _serde::__private::Some(
                            match __A::next_value::<Vec<Matrix4>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_initSkinTransform => {
                        if _serde::__private::Option::is_some(&m_initSkinTransform) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "initSkinTransform",
                                ),
                            );
                        }
                        m_initSkinTransform = _serde::__private::Some(
                            match __A::next_value::<Matrix4>(&mut __map) {
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
        }
        let m_mesh = match m_mesh {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("mesh"),
                );
            }
        };
        let m_nodeNames = match m_nodeNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nodeNames"),
                );
            }
        };
        let m_bindPose = match m_bindPose {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bindPose"),
                );
            }
        };
        let m_initSkinTransform = match m_initSkinTransform {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("initSkinTransform"),
                );
            }
        };
        _serde::__private::Ok(hkxSkinBinding {
            __ptr,
            parent,
            m_mesh,
            m_nodeNames,
            m_bindPose,
            m_initSkinTransform,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxSkinBinding<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "mesh",
                "nodeNames",
                "bindPose",
                "initSkinTransform",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxSkinBinding",
                FIELDS,
                __hkxSkinBindingVisitor {
                    marker: _serde::__private::PhantomData::<hkxSkinBinding>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
