use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxScene`
/// -         version: `1`
/// -       signature: `0x5f673ddd`
/// -          size: 176(x86)/224(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxScene<'a> {
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
    /// -          name: `modeller`(ctype: `hkStringPtr`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_modeller: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `asset`(ctype: `hkStringPtr`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_asset: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `sceneLength`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sceneLength: f32,
    /// # C++ Info
    /// -          name: `rootNode`(ctype: `struct hkxNode*`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_rootNode: Pointer,
    /// # C++ Info
    /// -          name: `selectionSets`(ctype: `hkArray<hkxNodeSelectionSet*>`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_selectionSets: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `cameras`(ctype: `hkArray<hkxCamera*>`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_cameras: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `lights`(ctype: `hkArray<hkxLight*>`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_lights: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `meshes`(ctype: `hkArray<hkxMesh*>`)
    /// -        offset:  60(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_meshes: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `materials`(ctype: `hkArray<hkxMaterial*>`)
    /// -        offset:  72(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_materials: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `inplaceTextures`(ctype: `hkArray<hkxTextureInplace*>`)
    /// -        offset:  84(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_inplaceTextures: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `externalTextures`(ctype: `hkArray<hkxTextureFile*>`)
    /// -        offset:  96(x86)/144(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_externalTextures: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `skinBindings`(ctype: `hkArray<hkxSkinBinding*>`)
    /// -        offset: 108(x86)/160(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_skinBindings: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `appliedTransform`(ctype: `hkMatrix3`)
    /// -        offset: 128(x86)/176(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_appliedTransform: Matrix3,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkxScene<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxScene"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5f673ddd)
        }
    }
    impl<'a> _serde::Serialize for hkxScene<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x5f673ddd)));
            let mut serializer = __serializer.serialize_struct("hkxScene", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("modeller", &self.m_modeller)?;
            serializer.serialize_stringptr_meta_field("asset", &self.m_asset)?;
            serializer.serialize_field("sceneLength", &self.m_sceneLength)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("rootNode", &self.m_rootNode)?;
            serializer
                .serialize_array_meta_field("selectionSets", &self.m_selectionSets)?;
            serializer.serialize_array_meta_field("cameras", &self.m_cameras)?;
            serializer.serialize_array_meta_field("lights", &self.m_lights)?;
            serializer.serialize_array_meta_field("meshes", &self.m_meshes)?;
            serializer.serialize_array_meta_field("materials", &self.m_materials)?;
            serializer
                .serialize_array_meta_field("inplaceTextures", &self.m_inplaceTextures)?;
            serializer
                .serialize_array_meta_field(
                    "externalTextures",
                    &self.m_externalTextures,
                )?;
            serializer.serialize_array_meta_field("skinBindings", &self.m_skinBindings)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("appliedTransform", &self.m_appliedTransform)?;
            serializer.serialize_stringptr_field("modeller", &self.m_modeller)?;
            serializer.serialize_stringptr_field("asset", &self.m_asset)?;
            serializer.serialize_array_field("selectionSets", &self.m_selectionSets)?;
            serializer.serialize_array_field("cameras", &self.m_cameras)?;
            serializer.serialize_array_field("lights", &self.m_lights)?;
            serializer.serialize_array_field("meshes", &self.m_meshes)?;
            serializer.serialize_array_field("materials", &self.m_materials)?;
            serializer
                .serialize_array_field("inplaceTextures", &self.m_inplaceTextures)?;
            serializer
                .serialize_array_field("externalTextures", &self.m_externalTextures)?;
            serializer.serialize_array_field("skinBindings", &self.m_skinBindings)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_modeller,
    m_asset,
    m_sceneLength,
    m_rootNode,
    m_selectionSets,
    m_cameras,
    m_lights,
    m_meshes,
    m_materials,
    m_inplaceTextures,
    m_externalTextures,
    m_skinBindings,
    m_appliedTransform,
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
            "modeller" => Ok(__Field::m_modeller),
            "asset" => Ok(__Field::m_asset),
            "sceneLength" => Ok(__Field::m_sceneLength),
            "rootNode" => Ok(__Field::m_rootNode),
            "selectionSets" => Ok(__Field::m_selectionSets),
            "cameras" => Ok(__Field::m_cameras),
            "lights" => Ok(__Field::m_lights),
            "meshes" => Ok(__Field::m_meshes),
            "materials" => Ok(__Field::m_materials),
            "inplaceTextures" => Ok(__Field::m_inplaceTextures),
            "externalTextures" => Ok(__Field::m_externalTextures),
            "skinBindings" => Ok(__Field::m_skinBindings),
            "appliedTransform" => Ok(__Field::m_appliedTransform),
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
pub(super) struct __hkxSceneVisitor<'de> {
    marker: core::marker::PhantomData<hkxScene<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkxSceneVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkxScene<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkxScene<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkxSceneVisitor<'de> {
    type Value = hkxScene<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkxScene")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __A::next_value(&mut __map)?;
        let mut m_modeller: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_asset: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_sceneLength: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_rootNode: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_selectionSets: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_cameras: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_lights: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_meshes: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_materials: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_inplaceTextures: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_externalTextures: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_skinBindings: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_appliedTransform: _serde::__private::Option<Matrix3> = _serde::__private::None;
        for i in 0..13usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_modeller) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "modeller",
                            ),
                        );
                    }
                    m_modeller = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_asset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("asset"),
                        );
                    }
                    m_asset = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_sceneLength) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sceneLength",
                            ),
                        );
                    }
                    m_sceneLength = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_rootNode) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rootNode",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_rootNode = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_selectionSets) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "selectionSets",
                            ),
                        );
                    }
                    m_selectionSets = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_cameras) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("cameras"),
                        );
                    }
                    m_cameras = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_lights) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("lights"),
                        );
                    }
                    m_lights = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_meshes) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("meshes"),
                        );
                    }
                    m_meshes = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_materials) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materials",
                            ),
                        );
                    }
                    m_materials = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_inplaceTextures) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "inplaceTextures",
                            ),
                        );
                    }
                    m_inplaceTextures = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_externalTextures) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "externalTextures",
                            ),
                        );
                    }
                    m_externalTextures = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_skinBindings) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "skinBindings",
                            ),
                        );
                    }
                    m_skinBindings = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_appliedTransform) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "appliedTransform",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 8usize, 0usize)?;
                    m_appliedTransform = _serde::__private::Some(
                        match __A::next_value::<Matrix3>(&mut __map) {
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
        let m_modeller = match m_modeller {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("modeller"),
                );
            }
        };
        let m_asset = match m_asset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("asset"),
                );
            }
        };
        let m_sceneLength = match m_sceneLength {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sceneLength"),
                );
            }
        };
        let m_rootNode = match m_rootNode {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rootNode"),
                );
            }
        };
        let m_selectionSets = match m_selectionSets {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("selectionSets"),
                );
            }
        };
        let m_cameras = match m_cameras {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cameras"),
                );
            }
        };
        let m_lights = match m_lights {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lights"),
                );
            }
        };
        let m_meshes = match m_meshes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("meshes"),
                );
            }
        };
        let m_materials = match m_materials {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materials"),
                );
            }
        };
        let m_inplaceTextures = match m_inplaceTextures {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("inplaceTextures"),
                );
            }
        };
        let m_externalTextures = match m_externalTextures {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("externalTextures"),
                );
            }
        };
        let m_skinBindings = match m_skinBindings {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("skinBindings"),
                );
            }
        };
        let m_appliedTransform = match m_appliedTransform {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("appliedTransform"),
                );
            }
        };
        _serde::__private::Ok(hkxScene {
            __ptr,
            parent,
            m_modeller,
            m_asset,
            m_sceneLength,
            m_rootNode,
            m_selectionSets,
            m_cameras,
            m_lights,
            m_meshes,
            m_materials,
            m_inplaceTextures,
            m_externalTextures,
            m_skinBindings,
            m_appliedTransform,
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
        let mut m_modeller: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_asset: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_sceneLength: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_rootNode: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_selectionSets: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_cameras: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_lights: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_meshes: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_materials: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_inplaceTextures: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_externalTextures: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_skinBindings: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_appliedTransform: _serde::__private::Option<Matrix3> = _serde::__private::None;
        for _ in 0..13usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_modeller => {
                        if _serde::__private::Option::is_some(&m_modeller) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "modeller",
                                ),
                            );
                        }
                        m_modeller = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_asset => {
                        if _serde::__private::Option::is_some(&m_asset) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("asset"),
                            );
                        }
                        m_asset = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_sceneLength => {
                        if _serde::__private::Option::is_some(&m_sceneLength) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "sceneLength",
                                ),
                            );
                        }
                        m_sceneLength = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_rootNode => {
                        if _serde::__private::Option::is_some(&m_rootNode) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rootNode",
                                ),
                            );
                        }
                        m_rootNode = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_selectionSets => {
                        if _serde::__private::Option::is_some(&m_selectionSets) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "selectionSets",
                                ),
                            );
                        }
                        m_selectionSets = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_cameras => {
                        if _serde::__private::Option::is_some(&m_cameras) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "cameras",
                                ),
                            );
                        }
                        m_cameras = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_lights => {
                        if _serde::__private::Option::is_some(&m_lights) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("lights"),
                            );
                        }
                        m_lights = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_meshes => {
                        if _serde::__private::Option::is_some(&m_meshes) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("meshes"),
                            );
                        }
                        m_meshes = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_materials => {
                        if _serde::__private::Option::is_some(&m_materials) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "materials",
                                ),
                            );
                        }
                        m_materials = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_inplaceTextures => {
                        if _serde::__private::Option::is_some(&m_inplaceTextures) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "inplaceTextures",
                                ),
                            );
                        }
                        m_inplaceTextures = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_externalTextures => {
                        if _serde::__private::Option::is_some(&m_externalTextures) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "externalTextures",
                                ),
                            );
                        }
                        m_externalTextures = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_skinBindings => {
                        if _serde::__private::Option::is_some(&m_skinBindings) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "skinBindings",
                                ),
                            );
                        }
                        m_skinBindings = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_appliedTransform => {
                        if _serde::__private::Option::is_some(&m_appliedTransform) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "appliedTransform",
                                ),
                            );
                        }
                        m_appliedTransform = _serde::__private::Some(
                            match __A::next_value::<Matrix3>(&mut __map) {
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
        let m_modeller = match m_modeller {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("modeller"),
                );
            }
        };
        let m_asset = match m_asset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("asset"),
                );
            }
        };
        let m_sceneLength = match m_sceneLength {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sceneLength"),
                );
            }
        };
        let m_rootNode = match m_rootNode {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rootNode"),
                );
            }
        };
        let m_selectionSets = match m_selectionSets {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("selectionSets"),
                );
            }
        };
        let m_cameras = match m_cameras {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cameras"),
                );
            }
        };
        let m_lights = match m_lights {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lights"),
                );
            }
        };
        let m_meshes = match m_meshes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("meshes"),
                );
            }
        };
        let m_materials = match m_materials {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materials"),
                );
            }
        };
        let m_inplaceTextures = match m_inplaceTextures {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("inplaceTextures"),
                );
            }
        };
        let m_externalTextures = match m_externalTextures {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("externalTextures"),
                );
            }
        };
        let m_skinBindings = match m_skinBindings {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("skinBindings"),
                );
            }
        };
        let m_appliedTransform = match m_appliedTransform {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("appliedTransform"),
                );
            }
        };
        _serde::__private::Ok(hkxScene {
            __ptr,
            parent,
            m_modeller,
            m_asset,
            m_sceneLength,
            m_rootNode,
            m_selectionSets,
            m_cameras,
            m_lights,
            m_meshes,
            m_materials,
            m_inplaceTextures,
            m_externalTextures,
            m_skinBindings,
            m_appliedTransform,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxScene<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "modeller",
                "asset",
                "sceneLength",
                "rootNode",
                "selectionSets",
                "cameras",
                "lights",
                "meshes",
                "materials",
                "inplaceTextures",
                "externalTextures",
                "skinBindings",
                "appliedTransform",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxScene",
                FIELDS,
                __hkxSceneVisitor {
                    marker: _serde::__private::PhantomData::<hkxScene>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
