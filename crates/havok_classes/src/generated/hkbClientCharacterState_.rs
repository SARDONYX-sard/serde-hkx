use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbClientCharacterState`
/// - version: `1`
/// - signature: `0xa2624c97`
/// - size: `208`(x86)/`272`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbClientCharacterState<'a> {
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
    /// - name: `deformableSkinIds`(ctype: `hkArray<hkUint64>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_deformableSkinIds: Vec<u64>,
    /// # C++ Info
    /// - name: `rigidSkinIds`(ctype: `hkArray<hkUint64>`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_rigidSkinIds: Vec<u64>,
    /// # C++ Info
    /// - name: `externalEventIds`(ctype: `hkArray<hkInt16>`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_externalEventIds: Vec<i16>,
    /// # C++ Info
    /// - name: `auxiliaryInfo`(ctype: `hkArray<hkbAuxiliaryNodeInfo*>`)
    /// - offset: ` 44`(x86)/` 64`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_auxiliaryInfo: Vec<Pointer>,
    /// # C++ Info
    /// - name: `activeEventIds`(ctype: `hkArray<hkInt16>`)
    /// - offset: ` 56`(x86)/` 80`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_activeEventIds: Vec<i16>,
    /// # C++ Info
    /// - name: `activeVariableIds`(ctype: `hkArray<hkInt16>`)
    /// - offset: ` 68`(x86)/` 96`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_activeVariableIds: Vec<i16>,
    /// # C++ Info
    /// - name: `characterId`(ctype: `hkUint64`)
    /// - offset: ` 80`(x86)/`112`(x86_64)
    /// - type_size: `  8`(x86)/`  8`(x86_64)
    pub m_characterId: u64,
    /// # C++ Info
    /// - name: `instanceName`(ctype: `hkStringPtr`)
    /// - offset: ` 88`(x86)/`120`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_instanceName: StringPtr<'a>,
    /// # C++ Info
    /// - name: `templateName`(ctype: `hkStringPtr`)
    /// - offset: ` 92`(x86)/`128`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_templateName: StringPtr<'a>,
    /// # C++ Info
    /// - name: `fullPathToProject`(ctype: `hkStringPtr`)
    /// - offset: ` 96`(x86)/`136`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_fullPathToProject: StringPtr<'a>,
    /// # C++ Info
    /// - name: `behaviorData`(ctype: `struct hkbBehaviorGraphData*`)
    /// - offset: `100`(x86)/`144`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_behaviorData: Pointer,
    /// # C++ Info
    /// - name: `behaviorInternalState`(ctype: `struct hkbBehaviorGraphInternalState*`)
    /// - offset: `104`(x86)/`152`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_behaviorInternalState: Pointer,
    /// # C++ Info
    /// - name: `nodeIdToInternalStateMap`(ctype: `void*`)
    /// - offset: `108`(x86)/`160`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_nodeIdToInternalStateMap: Pointer,
    /// # C++ Info
    /// - name: `visible`(ctype: `hkBool`)
    /// - offset: `112`(x86)/`168`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_visible: bool,
    /// # C++ Info
    /// - name: `elapsedSimulationTime`(ctype: `hkReal`)
    /// - offset: `116`(x86)/`172`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_elapsedSimulationTime: f32,
    /// # C++ Info
    /// - name: `skeleton`(ctype: `struct hkaSkeleton*`)
    /// - offset: `120`(x86)/`176`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_skeleton: Pointer,
    /// # C++ Info
    /// - name: `worldFromModel`(ctype: `hkQsTransform`)
    /// - offset: `128`(x86)/`192`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    pub m_worldFromModel: QsTransform,
    /// # C++ Info
    /// - name: `poseModelSpace`(ctype: `hkArray<hkQsTransform>`)
    /// - offset: `176`(x86)/`240`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_poseModelSpace: Vec<QsTransform>,
    /// # C++ Info
    /// - name: `rigidAttachmentTransforms`(ctype: `hkArray<hkQsTransform>`)
    /// - offset: `188`(x86)/`256`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_rigidAttachmentTransforms: Vec<QsTransform>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbClientCharacterState<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbClientCharacterState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa2624c97)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(self.m_auxiliaryInfo.iter().map(|ptr| ptr.get()));
            v.push(self.m_behaviorData.get());
            v.push(self.m_behaviorInternalState.get());
            v.push(self.m_nodeIdToInternalStateMap.get());
            v.push(self.m_skeleton.get());
            v
        }
    }
    impl<'a> _serde::Serialize for hkbClientCharacterState<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa2624c97)));
            let mut serializer = __serializer
                .serialize_struct("hkbClientCharacterState", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "deformableSkinIds",
                    &self.m_deformableSkinIds,
                )?;
            serializer.serialize_array_meta_field("rigidSkinIds", &self.m_rigidSkinIds)?;
            serializer
                .serialize_array_meta_field(
                    "externalEventIds",
                    &self.m_externalEventIds,
                )?;
            serializer
                .serialize_array_meta_field("auxiliaryInfo", &self.m_auxiliaryInfo)?;
            serializer
                .serialize_array_meta_field("activeEventIds", &self.m_activeEventIds)?;
            serializer
                .serialize_array_meta_field(
                    "activeVariableIds",
                    &self.m_activeVariableIds,
                )?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer
                .serialize_stringptr_meta_field("instanceName", &self.m_instanceName)?;
            serializer
                .serialize_stringptr_meta_field("templateName", &self.m_templateName)?;
            serializer
                .serialize_stringptr_meta_field(
                    "fullPathToProject",
                    &self.m_fullPathToProject,
                )?;
            serializer.serialize_field("behaviorData", &self.m_behaviorData)?;
            serializer
                .serialize_field(
                    "behaviorInternalState",
                    &self.m_behaviorInternalState,
                )?;
            serializer
                .skip_field(
                    "nodeIdToInternalStateMap",
                    &self.m_nodeIdToInternalStateMap,
                )?;
            serializer.serialize_field("visible", &self.m_visible)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "elapsedSimulationTime",
                    &self.m_elapsedSimulationTime,
                )?;
            serializer.serialize_field("skeleton", &self.m_skeleton)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("worldFromModel", &self.m_worldFromModel)?;
            serializer
                .serialize_array_meta_field("poseModelSpace", &self.m_poseModelSpace)?;
            serializer
                .serialize_array_meta_field(
                    "rigidAttachmentTransforms",
                    &self.m_rigidAttachmentTransforms,
                )?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_array_field("deformableSkinIds", &self.m_deformableSkinIds)?;
            serializer.serialize_array_field("rigidSkinIds", &self.m_rigidSkinIds)?;
            serializer
                .serialize_array_field("externalEventIds", &self.m_externalEventIds)?;
            serializer.serialize_array_field("auxiliaryInfo", &self.m_auxiliaryInfo)?;
            serializer.serialize_array_field("activeEventIds", &self.m_activeEventIds)?;
            serializer
                .serialize_array_field("activeVariableIds", &self.m_activeVariableIds)?;
            serializer.serialize_stringptr_field("instanceName", &self.m_instanceName)?;
            serializer.serialize_stringptr_field("templateName", &self.m_templateName)?;
            serializer
                .serialize_stringptr_field(
                    "fullPathToProject",
                    &self.m_fullPathToProject,
                )?;
            serializer.serialize_array_field("poseModelSpace", &self.m_poseModelSpace)?;
            serializer
                .serialize_array_field(
                    "rigidAttachmentTransforms",
                    &self.m_rigidAttachmentTransforms,
                )?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbClientCharacterState<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_deformableSkinIds,
                m_rigidSkinIds,
                m_externalEventIds,
                m_auxiliaryInfo,
                m_activeEventIds,
                m_activeVariableIds,
                m_characterId,
                m_instanceName,
                m_templateName,
                m_fullPathToProject,
                m_behaviorData,
                m_behaviorInternalState,
                m_visible,
                m_elapsedSimulationTime,
                m_skeleton,
                m_worldFromModel,
                m_poseModelSpace,
                m_rigidAttachmentTransforms,
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
                        "deformableSkinIds" => Ok(__Field::m_deformableSkinIds),
                        "rigidSkinIds" => Ok(__Field::m_rigidSkinIds),
                        "externalEventIds" => Ok(__Field::m_externalEventIds),
                        "auxiliaryInfo" => Ok(__Field::m_auxiliaryInfo),
                        "activeEventIds" => Ok(__Field::m_activeEventIds),
                        "activeVariableIds" => Ok(__Field::m_activeVariableIds),
                        "characterId" => Ok(__Field::m_characterId),
                        "instanceName" => Ok(__Field::m_instanceName),
                        "templateName" => Ok(__Field::m_templateName),
                        "fullPathToProject" => Ok(__Field::m_fullPathToProject),
                        "behaviorData" => Ok(__Field::m_behaviorData),
                        "behaviorInternalState" => Ok(__Field::m_behaviorInternalState),
                        "visible" => Ok(__Field::m_visible),
                        "elapsedSimulationTime" => Ok(__Field::m_elapsedSimulationTime),
                        "skeleton" => Ok(__Field::m_skeleton),
                        "worldFromModel" => Ok(__Field::m_worldFromModel),
                        "poseModelSpace" => Ok(__Field::m_poseModelSpace),
                        "rigidAttachmentTransforms" => {
                            Ok(__Field::m_rigidAttachmentTransforms)
                        }
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
            struct __hkbClientCharacterStateVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbClientCharacterState<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbClientCharacterStateVisitor<'de> {
                type Value = hkbClientCharacterState<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbClientCharacterState",
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
                    let mut m_deformableSkinIds: _serde::__private::Option<Vec<u64>> = _serde::__private::None;
                    let mut m_rigidSkinIds: _serde::__private::Option<Vec<u64>> = _serde::__private::None;
                    let mut m_externalEventIds: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
                    let mut m_auxiliaryInfo: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_activeEventIds: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
                    let mut m_activeVariableIds: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
                    let mut m_characterId: _serde::__private::Option<u64> = _serde::__private::None;
                    let mut m_instanceName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_templateName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_fullPathToProject: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_behaviorData: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_behaviorInternalState: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_nodeIdToInternalStateMap: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_visible: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_elapsedSimulationTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_skeleton: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_worldFromModel: _serde::__private::Option<QsTransform> = _serde::__private::None;
                    let mut m_poseModelSpace: _serde::__private::Option<
                        Vec<QsTransform>,
                    > = _serde::__private::None;
                    let mut m_rigidAttachmentTransforms: _serde::__private::Option<
                        Vec<QsTransform>,
                    > = _serde::__private::None;
                    for i in 0..19usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_deformableSkinIds,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deformableSkinIds",
                                        ),
                                    );
                                }
                                m_deformableSkinIds = _serde::__private::Some(
                                    match __A::next_value::<Vec<u64>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_rigidSkinIds) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidSkinIds",
                                        ),
                                    );
                                }
                                m_rigidSkinIds = _serde::__private::Some(
                                    match __A::next_value::<Vec<u64>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_externalEventIds) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "externalEventIds",
                                        ),
                                    );
                                }
                                m_externalEventIds = _serde::__private::Some(
                                    match __A::next_value::<Vec<i16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_auxiliaryInfo) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "auxiliaryInfo",
                                        ),
                                    );
                                }
                                m_auxiliaryInfo = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_activeEventIds) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "activeEventIds",
                                        ),
                                    );
                                }
                                m_activeEventIds = _serde::__private::Some(
                                    match __A::next_value::<Vec<i16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_activeVariableIds,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "activeVariableIds",
                                        ),
                                    );
                                }
                                m_activeVariableIds = _serde::__private::Some(
                                    match __A::next_value::<Vec<i16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_characterId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterId",
                                        ),
                                    );
                                }
                                m_characterId = _serde::__private::Some(
                                    match __A::next_value::<u64>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_instanceName) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "instanceName",
                                        ),
                                    );
                                }
                                m_instanceName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_templateName) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "templateName",
                                        ),
                                    );
                                }
                                m_templateName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_fullPathToProject,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fullPathToProject",
                                        ),
                                    );
                                }
                                m_fullPathToProject = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_behaviorData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "behaviorData",
                                        ),
                                    );
                                }
                                m_behaviorData = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(
                                    &m_behaviorInternalState,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "behaviorInternalState",
                                        ),
                                    );
                                }
                                m_behaviorInternalState = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(
                                    &m_nodeIdToInternalStateMap,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nodeIdToInternalStateMap",
                                        ),
                                    );
                                }
                                m_nodeIdToInternalStateMap = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_visible) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "visible",
                                        ),
                                    );
                                }
                                m_visible = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_elapsedSimulationTime,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "elapsedSimulationTime",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 3usize)?;
                                m_elapsedSimulationTime = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(&m_skeleton) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skeleton",
                                        ),
                                    );
                                }
                                m_skeleton = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(&m_worldFromModel) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "worldFromModel",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 4usize, 8usize)?;
                                m_worldFromModel = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(&m_poseModelSpace) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "poseModelSpace",
                                        ),
                                    );
                                }
                                m_poseModelSpace = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_rigidAttachmentTransforms,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidAttachmentTransforms",
                                        ),
                                    );
                                }
                                m_rigidAttachmentTransforms = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
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
                    __A::pad(&mut __map, 8usize, 0usize)?;
                    let m_deformableSkinIds = match m_deformableSkinIds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deformableSkinIds",
                                ),
                            );
                        }
                    };
                    let m_rigidSkinIds = match m_rigidSkinIds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidSkinIds",
                                ),
                            );
                        }
                    };
                    let m_externalEventIds = match m_externalEventIds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "externalEventIds",
                                ),
                            );
                        }
                    };
                    let m_auxiliaryInfo = match m_auxiliaryInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "auxiliaryInfo",
                                ),
                            );
                        }
                    };
                    let m_activeEventIds = match m_activeEventIds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "activeEventIds",
                                ),
                            );
                        }
                    };
                    let m_activeVariableIds = match m_activeVariableIds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "activeVariableIds",
                                ),
                            );
                        }
                    };
                    let m_characterId = match m_characterId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterId",
                                ),
                            );
                        }
                    };
                    let m_instanceName = match m_instanceName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "instanceName",
                                ),
                            );
                        }
                    };
                    let m_templateName = match m_templateName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "templateName",
                                ),
                            );
                        }
                    };
                    let m_fullPathToProject = match m_fullPathToProject {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fullPathToProject",
                                ),
                            );
                        }
                    };
                    let m_behaviorData = match m_behaviorData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "behaviorData",
                                ),
                            );
                        }
                    };
                    let m_behaviorInternalState = match m_behaviorInternalState {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "behaviorInternalState",
                                ),
                            );
                        }
                    };
                    let m_nodeIdToInternalStateMap = match m_nodeIdToInternalStateMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nodeIdToInternalStateMap",
                                ),
                            );
                        }
                    };
                    let m_visible = match m_visible {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("visible"),
                            );
                        }
                    };
                    let m_elapsedSimulationTime = match m_elapsedSimulationTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "elapsedSimulationTime",
                                ),
                            );
                        }
                    };
                    let m_skeleton = match m_skeleton {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("skeleton"),
                            );
                        }
                    };
                    let m_worldFromModel = match m_worldFromModel {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "worldFromModel",
                                ),
                            );
                        }
                    };
                    let m_poseModelSpace = match m_poseModelSpace {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "poseModelSpace",
                                ),
                            );
                        }
                    };
                    let m_rigidAttachmentTransforms = match m_rigidAttachmentTransforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidAttachmentTransforms",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbClientCharacterState {
                        __ptr,
                        parent,
                        m_deformableSkinIds,
                        m_rigidSkinIds,
                        m_externalEventIds,
                        m_auxiliaryInfo,
                        m_activeEventIds,
                        m_activeVariableIds,
                        m_characterId,
                        m_instanceName,
                        m_templateName,
                        m_fullPathToProject,
                        m_behaviorData,
                        m_behaviorInternalState,
                        m_nodeIdToInternalStateMap,
                        m_visible,
                        m_elapsedSimulationTime,
                        m_skeleton,
                        m_worldFromModel,
                        m_poseModelSpace,
                        m_rigidAttachmentTransforms,
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
                    let mut m_deformableSkinIds: _serde::__private::Option<Vec<u64>> = _serde::__private::None;
                    let mut m_rigidSkinIds: _serde::__private::Option<Vec<u64>> = _serde::__private::None;
                    let mut m_externalEventIds: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
                    let mut m_auxiliaryInfo: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_activeEventIds: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
                    let mut m_activeVariableIds: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
                    let mut m_characterId: _serde::__private::Option<u64> = _serde::__private::None;
                    let mut m_instanceName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_templateName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_fullPathToProject: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_behaviorData: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_behaviorInternalState: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_visible: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_elapsedSimulationTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_skeleton: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_worldFromModel: _serde::__private::Option<QsTransform> = _serde::__private::None;
                    let mut m_poseModelSpace: _serde::__private::Option<
                        Vec<QsTransform>,
                    > = _serde::__private::None;
                    let mut m_rigidAttachmentTransforms: _serde::__private::Option<
                        Vec<QsTransform>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_deformableSkinIds => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_deformableSkinIds,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deformableSkinIds",
                                        ),
                                    );
                                }
                                m_deformableSkinIds = _serde::__private::Some(
                                    match __A::next_value::<Vec<u64>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_rigidSkinIds => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_rigidSkinIds) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidSkinIds",
                                        ),
                                    );
                                }
                                m_rigidSkinIds = _serde::__private::Some(
                                    match __A::next_value::<Vec<u64>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_externalEventIds => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_externalEventIds) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "externalEventIds",
                                        ),
                                    );
                                }
                                m_externalEventIds = _serde::__private::Some(
                                    match __A::next_value::<Vec<i16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_auxiliaryInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_auxiliaryInfo) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "auxiliaryInfo",
                                        ),
                                    );
                                }
                                m_auxiliaryInfo = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_activeEventIds => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_activeEventIds) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "activeEventIds",
                                        ),
                                    );
                                }
                                m_activeEventIds = _serde::__private::Some(
                                    match __A::next_value::<Vec<i16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_activeVariableIds => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_activeVariableIds,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "activeVariableIds",
                                        ),
                                    );
                                }
                                m_activeVariableIds = _serde::__private::Some(
                                    match __A::next_value::<Vec<i16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_characterId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_characterId) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterId",
                                        ),
                                    );
                                }
                                m_characterId = _serde::__private::Some(
                                    match __A::next_value::<u64>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_instanceName => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_instanceName) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "instanceName",
                                        ),
                                    );
                                }
                                m_instanceName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_templateName => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_templateName) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "templateName",
                                        ),
                                    );
                                }
                                m_templateName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_fullPathToProject => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_fullPathToProject,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fullPathToProject",
                                        ),
                                    );
                                }
                                m_fullPathToProject = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_behaviorData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_behaviorData) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "behaviorData",
                                        ),
                                    );
                                }
                                m_behaviorData = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_behaviorInternalState => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_behaviorInternalState,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "behaviorInternalState",
                                        ),
                                    );
                                }
                                m_behaviorInternalState = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_visible => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_visible) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "visible",
                                        ),
                                    );
                                }
                                m_visible = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_elapsedSimulationTime => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_elapsedSimulationTime,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "elapsedSimulationTime",
                                        ),
                                    );
                                }
                                m_elapsedSimulationTime = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_skeleton => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_skeleton) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skeleton",
                                        ),
                                    );
                                }
                                m_skeleton = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_worldFromModel => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_worldFromModel) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "worldFromModel",
                                        ),
                                    );
                                }
                                m_worldFromModel = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_poseModelSpace => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_poseModelSpace) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "poseModelSpace",
                                        ),
                                    );
                                }
                                m_poseModelSpace = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_rigidAttachmentTransforms => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_rigidAttachmentTransforms,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidAttachmentTransforms",
                                        ),
                                    );
                                }
                                m_rigidAttachmentTransforms = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
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
                    let m_deformableSkinIds = match m_deformableSkinIds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deformableSkinIds",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rigidSkinIds = match m_rigidSkinIds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidSkinIds",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_externalEventIds = match m_externalEventIds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "externalEventIds",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_auxiliaryInfo = match m_auxiliaryInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "auxiliaryInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_activeEventIds = match m_activeEventIds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "activeEventIds",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_activeVariableIds = match m_activeVariableIds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "activeVariableIds",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_characterId = match m_characterId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_instanceName = match m_instanceName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "instanceName",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_templateName = match m_templateName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "templateName",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fullPathToProject = match m_fullPathToProject {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fullPathToProject",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_behaviorData = match m_behaviorData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "behaviorData",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_behaviorInternalState = match m_behaviorInternalState {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "behaviorInternalState",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_visible = match m_visible {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("visible"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_elapsedSimulationTime = match m_elapsedSimulationTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "elapsedSimulationTime",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_skeleton = match m_skeleton {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("skeleton"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_worldFromModel = match m_worldFromModel {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "worldFromModel",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_poseModelSpace = match m_poseModelSpace {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "poseModelSpace",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rigidAttachmentTransforms = match m_rigidAttachmentTransforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidAttachmentTransforms",
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbClientCharacterState {
                        __ptr,
                        parent,
                        m_deformableSkinIds,
                        m_rigidSkinIds,
                        m_externalEventIds,
                        m_auxiliaryInfo,
                        m_activeEventIds,
                        m_activeVariableIds,
                        m_characterId,
                        m_instanceName,
                        m_templateName,
                        m_fullPathToProject,
                        m_behaviorData,
                        m_behaviorInternalState,
                        m_visible,
                        m_elapsedSimulationTime,
                        m_skeleton,
                        m_worldFromModel,
                        m_poseModelSpace,
                        m_rigidAttachmentTransforms,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "deformableSkinIds",
                "rigidSkinIds",
                "externalEventIds",
                "auxiliaryInfo",
                "activeEventIds",
                "activeVariableIds",
                "characterId",
                "instanceName",
                "templateName",
                "fullPathToProject",
                "behaviorData",
                "behaviorInternalState",
                "nodeIdToInternalStateMap",
                "visible",
                "elapsedSimulationTime",
                "skeleton",
                "worldFromModel",
                "poseModelSpace",
                "rigidAttachmentTransforms",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbClientCharacterState",
                FIELDS,
                __hkbClientCharacterStateVisitor {
                    marker: _serde::__private::PhantomData::<hkbClientCharacterState>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
