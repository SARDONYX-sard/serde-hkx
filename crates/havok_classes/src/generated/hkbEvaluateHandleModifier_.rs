use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbEvaluateHandleModifier`
/// - version: `2`
/// - signature: `0x79757102`
/// - size: `176`(x86)/`240`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbEvaluateHandleModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// - name: `handle`(ctype: `struct hkbHandle*`)
    /// - offset: ` 44`(x86)/` 80`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "handle"))]
    #[cfg_attr(feature = "serde", serde(rename = "handle"))]
    pub m_handle: Pointer,
    /// # C++ Info
    /// - name: `handlePositionOut`(ctype: `hkVector4`)
    /// - offset: ` 48`(x86)/` 96`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "handlePositionOut"))]
    #[cfg_attr(feature = "serde", serde(rename = "handlePositionOut"))]
    pub m_handlePositionOut: Vector4,
    /// # C++ Info
    /// - name: `handleRotationOut`(ctype: `hkQuaternion`)
    /// - offset: ` 64`(x86)/`112`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "handleRotationOut"))]
    #[cfg_attr(feature = "serde", serde(rename = "handleRotationOut"))]
    pub m_handleRotationOut: Quaternion,
    /// # C++ Info
    /// - name: `isValidOut`(ctype: `hkBool`)
    /// - offset: ` 80`(x86)/`128`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "isValidOut"))]
    #[cfg_attr(feature = "serde", serde(rename = "isValidOut"))]
    pub m_isValidOut: bool,
    /// # C++ Info
    /// - name: `extrapolationTimeStep`(ctype: `hkReal`)
    /// - offset: ` 84`(x86)/`132`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "extrapolationTimeStep"))]
    #[cfg_attr(feature = "serde", serde(rename = "extrapolationTimeStep"))]
    pub m_extrapolationTimeStep: f32,
    /// # C++ Info
    /// - name: `handleChangeSpeed`(ctype: `hkReal`)
    /// - offset: ` 88`(x86)/`136`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "handleChangeSpeed"))]
    #[cfg_attr(feature = "serde", serde(rename = "handleChangeSpeed"))]
    pub m_handleChangeSpeed: f32,
    /// # C++ Info
    /// - name: `handleChangeMode`(ctype: `enum HandleChangeMode`)
    /// - offset: ` 92`(x86)/`140`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "handleChangeMode"))]
    #[cfg_attr(feature = "serde", serde(rename = "handleChangeMode"))]
    pub m_handleChangeMode: HandleChangeMode,
    /// # C++ Info
    /// - name: `oldHandle`(ctype: `struct hkbHandle`)
    /// - offset: ` 96`(x86)/`144`(x86_64)
    /// - type_size: ` 24`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "oldHandle"))]
    #[cfg_attr(feature = "serde", serde(rename = "oldHandle"))]
    pub m_oldHandle: hkbHandle,
    /// # C++ Info
    /// - name: `oldHandlePosition`(ctype: `hkVector4`)
    /// - offset: `128`(x86)/`192`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "oldHandlePosition"))]
    #[cfg_attr(feature = "serde", serde(rename = "oldHandlePosition"))]
    pub m_oldHandlePosition: Vector4,
    /// # C++ Info
    /// - name: `oldHandleRotation`(ctype: `hkQuaternion`)
    /// - offset: `144`(x86)/`208`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "oldHandleRotation"))]
    #[cfg_attr(feature = "serde", serde(rename = "oldHandleRotation"))]
    pub m_oldHandleRotation: Quaternion,
    /// # C++ Info
    /// - name: `timeSinceLastModify`(ctype: `hkReal`)
    /// - offset: `160`(x86)/`224`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeSinceLastModify"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeSinceLastModify"))]
    pub m_timeSinceLastModify: f32,
    /// # C++ Info
    /// - name: `smoothlyChangingHandles`(ctype: `hkBool`)
    /// - offset: `164`(x86)/`228`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "smoothlyChangingHandles"))]
    #[cfg_attr(feature = "serde", serde(rename = "smoothlyChangingHandles"))]
    pub m_smoothlyChangingHandles: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbEvaluateHandleModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbEvaluateHandleModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x79757102)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.parent.parent.parent.m_variableBindingSet.get());
            v.push(self.m_handle.get());
            v.extend(self.m_oldHandle.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hkbEvaluateHandleModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x79757102)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbEvaluateHandleModifier",
                    class_meta,
                    (176u64, 240u64),
                )?;
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
                .skip_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.serialize_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer
                .skip_fixed_array_field(
                    "padNode",
                    self.parent.parent.m_padNode.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer
                .skip_fixed_array_field(
                    "padModifier",
                    self.parent.m_padModifier.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("handle", &self.m_handle)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("handlePositionOut", &self.m_handlePositionOut)?;
            serializer.serialize_field("handleRotationOut", &self.m_handleRotationOut)?;
            serializer.serialize_field("isValidOut", &self.m_isValidOut)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "extrapolationTimeStep",
                    &self.m_extrapolationTimeStep,
                )?;
            serializer.serialize_field("handleChangeSpeed", &self.m_handleChangeSpeed)?;
            serializer.serialize_field("handleChangeMode", &self.m_handleChangeMode)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("oldHandle", &self.m_oldHandle)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.skip_field("oldHandlePosition", &self.m_oldHandlePosition)?;
            serializer.skip_field("oldHandleRotation", &self.m_oldHandleRotation)?;
            serializer.skip_field("timeSinceLastModify", &self.m_timeSinceLastModify)?;
            serializer
                .skip_field("smoothlyChangingHandles", &self.m_smoothlyChangingHandles)?;
            serializer.pad_field([0u8; 11usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbEvaluateHandleModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_enable,
                m_handle,
                m_handlePositionOut,
                m_handleRotationOut,
                m_isValidOut,
                m_extrapolationTimeStep,
                m_handleChangeSpeed,
                m_handleChangeMode,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "variableBindingSet" => Ok(__Field::m_variableBindingSet),
                        "userData" => Ok(__Field::m_userData),
                        "name" => Ok(__Field::m_name),
                        "enable" => Ok(__Field::m_enable),
                        "handle" => Ok(__Field::m_handle),
                        "handlePositionOut" => Ok(__Field::m_handlePositionOut),
                        "handleRotationOut" => Ok(__Field::m_handleRotationOut),
                        "isValidOut" => Ok(__Field::m_isValidOut),
                        "extrapolationTimeStep" => Ok(__Field::m_extrapolationTimeStep),
                        "handleChangeSpeed" => Ok(__Field::m_handleChangeSpeed),
                        "handleChangeMode" => Ok(__Field::m_handleChangeMode),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hkbEvaluateHandleModifierVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbEvaluateHandleModifier<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbEvaluateHandleModifierVisitor<'de> {
                type Value = hkbEvaluateHandleModifier<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbEvaluateHandleModifier",
                    )
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
                    let mut m_handle: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_handlePositionOut: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_handleRotationOut: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_isValidOut: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_extrapolationTimeStep: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_handleChangeSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_handleChangeMode: _serde::__private::Option<
                        HandleChangeMode,
                    > = _serde::__private::None;
                    let mut m_oldHandle: _serde::__private::Option<hkbHandle> = _serde::__private::None;
                    let mut m_oldHandlePosition: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_oldHandleRotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_timeSinceLastModify: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_smoothlyChangingHandles: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..12usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_handle) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("handle"),
                                    );
                                }
                                m_handle = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_handlePositionOut,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "handlePositionOut",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 8usize)?;
                                m_handlePositionOut = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_handleRotationOut,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "handleRotationOut",
                                        ),
                                    );
                                }
                                m_handleRotationOut = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_isValidOut) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isValidOut",
                                        ),
                                    );
                                }
                                m_isValidOut = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_extrapolationTimeStep,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extrapolationTimeStep",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 3usize)?;
                                m_extrapolationTimeStep = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_handleChangeSpeed,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "handleChangeSpeed",
                                        ),
                                    );
                                }
                                m_handleChangeSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_handleChangeMode) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "handleChangeMode",
                                        ),
                                    );
                                }
                                m_handleChangeMode = _serde::__private::Some(
                                    match __A::next_value::<HandleChangeMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_oldHandle) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "oldHandle",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 3usize)?;
                                m_oldHandle = _serde::__private::Some(
                                    match __A::next_value::<hkbHandle>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_oldHandlePosition,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "oldHandlePosition",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 8usize, 0usize)?;
                                m_oldHandlePosition = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_oldHandleRotation,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "oldHandleRotation",
                                        ),
                                    );
                                }
                                m_oldHandleRotation = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_timeSinceLastModify,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeSinceLastModify",
                                        ),
                                    );
                                }
                                m_timeSinceLastModify = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(
                                    &m_smoothlyChangingHandles,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "smoothlyChangingHandles",
                                        ),
                                    );
                                }
                                m_smoothlyChangingHandles = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                    __A::pad(&mut __map, 11usize, 11usize)?;
                    let m_handle = match m_handle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("handle"),
                            );
                        }
                    };
                    let m_handlePositionOut = match m_handlePositionOut {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "handlePositionOut",
                                ),
                            );
                        }
                    };
                    let m_handleRotationOut = match m_handleRotationOut {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "handleRotationOut",
                                ),
                            );
                        }
                    };
                    let m_isValidOut = match m_isValidOut {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isValidOut",
                                ),
                            );
                        }
                    };
                    let m_extrapolationTimeStep = match m_extrapolationTimeStep {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "extrapolationTimeStep",
                                ),
                            );
                        }
                    };
                    let m_handleChangeSpeed = match m_handleChangeSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "handleChangeSpeed",
                                ),
                            );
                        }
                    };
                    let m_handleChangeMode = match m_handleChangeMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "handleChangeMode",
                                ),
                            );
                        }
                    };
                    let m_oldHandle = match m_oldHandle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "oldHandle",
                                ),
                            );
                        }
                    };
                    let m_oldHandlePosition = match m_oldHandlePosition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "oldHandlePosition",
                                ),
                            );
                        }
                    };
                    let m_oldHandleRotation = match m_oldHandleRotation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "oldHandleRotation",
                                ),
                            );
                        }
                    };
                    let m_timeSinceLastModify = match m_timeSinceLastModify {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeSinceLastModify",
                                ),
                            );
                        }
                    };
                    let m_smoothlyChangingHandles = match m_smoothlyChangingHandles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "smoothlyChangingHandles",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbEvaluateHandleModifier {
                        __ptr,
                        parent,
                        m_handle,
                        m_handlePositionOut,
                        m_handleRotationOut,
                        m_isValidOut,
                        m_extrapolationTimeStep,
                        m_handleChangeSpeed,
                        m_handleChangeMode,
                        m_oldHandle,
                        m_oldHandlePosition,
                        m_oldHandleRotation,
                        m_timeSinceLastModify,
                        m_smoothlyChangingHandles,
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
                    let mut m_variableBindingSet: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_enable: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_handle: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_handlePositionOut: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_handleRotationOut: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_isValidOut: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_extrapolationTimeStep: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_handleChangeSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_handleChangeMode: _serde::__private::Option<
                        HandleChangeMode,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_variableBindingSet => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_variableBindingSet,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableBindingSet",
                                        ),
                                    );
                                }
                                m_variableBindingSet = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<Ulong>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_enable => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_enable) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("enable"),
                                    );
                                }
                                m_enable = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_handle => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_handle) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("handle"),
                                    );
                                }
                                m_handle = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_handlePositionOut => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_handlePositionOut,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "handlePositionOut",
                                        ),
                                    );
                                }
                                m_handlePositionOut = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_handleRotationOut => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_handleRotationOut,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "handleRotationOut",
                                        ),
                                    );
                                }
                                m_handleRotationOut = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isValidOut => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isValidOut) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isValidOut",
                                        ),
                                    );
                                }
                                m_isValidOut = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_extrapolationTimeStep => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_extrapolationTimeStep,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extrapolationTimeStep",
                                        ),
                                    );
                                }
                                m_extrapolationTimeStep = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_handleChangeSpeed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_handleChangeSpeed,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "handleChangeSpeed",
                                        ),
                                    );
                                }
                                m_handleChangeSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_handleChangeMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_handleChangeMode) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "handleChangeMode",
                                        ),
                                    );
                                }
                                m_handleChangeMode = _serde::__private::Some(
                                    match __A::next_value::<HandleChangeMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_variableBindingSet = match m_variableBindingSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableBindingSet",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enable = match m_enable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("enable"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_handle = match m_handle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("handle"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_handlePositionOut = match m_handlePositionOut {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "handlePositionOut",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_handleRotationOut = match m_handleRotationOut {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "handleRotationOut",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isValidOut = match m_isValidOut {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isValidOut",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_extrapolationTimeStep = match m_extrapolationTimeStep {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "extrapolationTimeStep",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_handleChangeSpeed = match m_handleChangeSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "handleChangeSpeed",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_handleChangeMode = match m_handleChangeMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "handleChangeMode",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let parent = hkbBindable {
                        __ptr,
                        parent,
                        m_variableBindingSet,
                        ..Default::default()
                    };
                    let parent = hkbNode {
                        __ptr,
                        parent,
                        m_userData,
                        m_name,
                        ..Default::default()
                    };
                    let parent = hkbModifier {
                        __ptr,
                        parent,
                        m_enable,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbEvaluateHandleModifier {
                        __ptr,
                        parent,
                        m_handle,
                        m_handlePositionOut,
                        m_handleRotationOut,
                        m_isValidOut,
                        m_extrapolationTimeStep,
                        m_handleChangeSpeed,
                        m_handleChangeMode,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "handle",
                "handlePositionOut",
                "handleRotationOut",
                "isValidOut",
                "extrapolationTimeStep",
                "handleChangeSpeed",
                "handleChangeMode",
                "oldHandle",
                "oldHandlePosition",
                "oldHandleRotation",
                "timeSinceLastModify",
                "smoothlyChangingHandles",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbEvaluateHandleModifier",
                FIELDS,
                __hkbEvaluateHandleModifierVisitor {
                    marker: _serde::__private::PhantomData::<hkbEvaluateHandleModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
pub enum HandleChangeMode {
    #[default]
    HANDLE_CHANGE_MODE_ABRUPT = 0isize,
    HANDLE_CHANGE_MODE_CONSTANT_VELOCITY = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for HandleChangeMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HANDLE_CHANGE_MODE_ABRUPT => {
                    __serializer.serialize_field("HANDLE_CHANGE_MODE_ABRUPT", &0u64)
                }
                Self::HANDLE_CHANGE_MODE_CONSTANT_VELOCITY => {
                    __serializer
                        .serialize_field("HANDLE_CHANGE_MODE_CONSTANT_VELOCITY", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum HandleChangeMode to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for HandleChangeMode {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v.eq_ignore_ascii_case("HANDLE_CHANGE_MODE_ABRUPT") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case(
                                        "HANDLE_CHANGE_MODE_CONSTANT_VELOCITY",
                                    ) => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<HandleChangeMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = HandleChangeMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum HandleChangeMode",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                HandleChangeMode::HANDLE_CHANGE_MODE_ABRUPT,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                HandleChangeMode::HANDLE_CHANGE_MODE_CONSTANT_VELOCITY,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "HANDLE_CHANGE_MODE_ABRUPT",
                "HANDLE_CHANGE_MODE_CONSTANT_VELOCITY",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "HandleChangeMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<HandleChangeMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
