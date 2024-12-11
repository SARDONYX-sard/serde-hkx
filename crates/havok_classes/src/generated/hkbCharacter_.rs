use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbCharacter`
/// - version: `2`
/// - signature: `0x3088a5c5`
/// - size: ` 88`(x86)/`160`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacter<'a> {
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
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// - name: `nearbyCharacters`(ctype: `hkArray<hkbCharacter*>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "nearbyCharacters"))]
    #[cfg_attr(feature = "serde", serde(rename = "nearbyCharacters"))]
    pub m_nearbyCharacters: Vec<Pointer>,
    /// # C++ Info
    /// - name: `currentLod`(ctype: `hkInt16`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentLod"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentLod"))]
    pub m_currentLod: i16,
    /// # C++ Info
    /// - name: `numTracksInLod`(ctype: `hkInt16`)
    /// - offset: ` 22`(x86)/` 34`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "numTracksInLod"))]
    #[cfg_attr(feature = "serde", serde(rename = "numTracksInLod"))]
    pub m_numTracksInLod: i16,
    /// # C++ Info
    /// - name: `name`(ctype: `hkStringPtr`)
    /// - offset: ` 24`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// - name: `ragdollDriver`(ctype: `void*`)
    /// - offset: ` 28`(x86)/` 48`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "ragdollDriver"))]
    #[cfg_attr(feature = "serde", serde(rename = "ragdollDriver"))]
    pub m_ragdollDriver: Pointer,
    /// # C++ Info
    /// - name: `characterControllerDriver`(ctype: `void*`)
    /// - offset: ` 32`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "characterControllerDriver"))]
    #[cfg_attr(feature = "serde", serde(rename = "characterControllerDriver"))]
    pub m_characterControllerDriver: Pointer,
    /// # C++ Info
    /// - name: `footIkDriver`(ctype: `void*`)
    /// - offset: ` 36`(x86)/` 64`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "footIkDriver"))]
    #[cfg_attr(feature = "serde", serde(rename = "footIkDriver"))]
    pub m_footIkDriver: Pointer,
    /// # C++ Info
    /// - name: `handIkDriver`(ctype: `void*`)
    /// - offset: ` 40`(x86)/` 72`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "handIkDriver"))]
    #[cfg_attr(feature = "serde", serde(rename = "handIkDriver"))]
    pub m_handIkDriver: Pointer,
    /// # C++ Info
    /// - name: `setup`(ctype: `struct hkbCharacterSetup*`)
    /// - offset: ` 44`(x86)/` 80`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "setup"))]
    #[cfg_attr(feature = "serde", serde(rename = "setup"))]
    pub m_setup: Pointer,
    /// # C++ Info
    /// - name: `behaviorGraph`(ctype: `struct hkbBehaviorGraph*`)
    /// - offset: ` 48`(x86)/` 88`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "behaviorGraph"))]
    #[cfg_attr(feature = "serde", serde(rename = "behaviorGraph"))]
    pub m_behaviorGraph: Pointer,
    /// # C++ Info
    /// - name: `projectData`(ctype: `struct hkbProjectData*`)
    /// - offset: ` 52`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "projectData"))]
    #[cfg_attr(feature = "serde", serde(rename = "projectData"))]
    pub m_projectData: Pointer,
    /// # C++ Info
    /// - name: `animationBindingSet`(ctype: `void*`)
    /// - offset: ` 56`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "animationBindingSet"))]
    #[cfg_attr(feature = "serde", serde(rename = "animationBindingSet"))]
    pub m_animationBindingSet: Pointer,
    /// # C++ Info
    /// - name: `raycastInterface`(ctype: `void*`)
    /// - offset: ` 60`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "raycastInterface"))]
    #[cfg_attr(feature = "serde", serde(rename = "raycastInterface"))]
    pub m_raycastInterface: Pointer,
    /// # C++ Info
    /// - name: `world`(ctype: `void*`)
    /// - offset: ` 64`(x86)/`120`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `NOT_OWNED|SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "world"))]
    #[cfg_attr(feature = "serde", serde(rename = "world"))]
    pub m_world: Pointer,
    /// # C++ Info
    /// - name: `eventQueue`(ctype: `void*`)
    /// - offset: ` 68`(x86)/`128`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "eventQueue"))]
    #[cfg_attr(feature = "serde", serde(rename = "eventQueue"))]
    pub m_eventQueue: Pointer,
    /// # C++ Info
    /// - name: `worldFromModel`(ctype: `void*`)
    /// - offset: ` 72`(x86)/`136`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "worldFromModel"))]
    #[cfg_attr(feature = "serde", serde(rename = "worldFromModel"))]
    pub m_worldFromModel: Pointer,
    /// # C++ Info
    /// - name: `poseLocal`(ctype: `hkSimpleArray<void>`)
    /// - offset: ` 76`(x86)/`144`(x86_64)
    /// - type_size: `  8`(x86)/` 12`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "poseLocal"))]
    #[cfg_attr(feature = "serde", serde(rename = "poseLocal"))]
    pub m_poseLocal: Vec<()>,
    /// # C++ Info
    /// - name: `deleteWorldFromModel`(ctype: `hkBool`)
    /// - offset: ` 84`(x86)/`156`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "deleteWorldFromModel"))]
    #[cfg_attr(feature = "serde", serde(rename = "deleteWorldFromModel"))]
    pub m_deleteWorldFromModel: bool,
    /// # C++ Info
    /// - name: `deletePoseLocal`(ctype: `hkBool`)
    /// - offset: ` 85`(x86)/`157`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "deletePoseLocal"))]
    #[cfg_attr(feature = "serde", serde(rename = "deletePoseLocal"))]
    pub m_deletePoseLocal: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCharacter<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacter"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x3088a5c5)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(self.m_nearbyCharacters.iter().map(|ptr| ptr.get()));
            v.push(self.m_ragdollDriver.get());
            v.push(self.m_characterControllerDriver.get());
            v.push(self.m_footIkDriver.get());
            v.push(self.m_handIkDriver.get());
            v.push(self.m_setup.get());
            v.push(self.m_behaviorGraph.get());
            v.push(self.m_projectData.get());
            v.push(self.m_animationBindingSet.get());
            v.push(self.m_raycastInterface.get());
            v.push(self.m_world.get());
            v.push(self.m_eventQueue.get());
            v.push(self.m_worldFromModel.get());
            v
        }
    }
    impl<'a> _serde::Serialize for hkbCharacter<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x3088a5c5)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacter", class_meta, (88u64, 160u64))?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "nearbyCharacters",
                    &self.m_nearbyCharacters,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("currentLod", &self.m_currentLod)?;
            serializer.skip_field("numTracksInLod", &self.m_numTracksInLod)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer.skip_field("ragdollDriver", &self.m_ragdollDriver)?;
            serializer
                .skip_field(
                    "characterControllerDriver",
                    &self.m_characterControllerDriver,
                )?;
            serializer.skip_field("footIkDriver", &self.m_footIkDriver)?;
            serializer.skip_field("handIkDriver", &self.m_handIkDriver)?;
            serializer.serialize_field("setup", &self.m_setup)?;
            serializer.serialize_field("behaviorGraph", &self.m_behaviorGraph)?;
            serializer.serialize_field("projectData", &self.m_projectData)?;
            serializer.skip_field("animationBindingSet", &self.m_animationBindingSet)?;
            serializer.skip_field("raycastInterface", &self.m_raycastInterface)?;
            serializer.skip_field("world", &self.m_world)?;
            serializer.skip_field("eventQueue", &self.m_eventQueue)?;
            serializer.skip_field("worldFromModel", &self.m_worldFromModel)?;
            serializer.skip_field("poseLocal", &self.m_poseLocal)?;
            serializer.skip_field("deleteWorldFromModel", &self.m_deleteWorldFromModel)?;
            serializer.skip_field("deletePoseLocal", &self.m_deletePoseLocal)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbCharacter<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_nearbyCharacters,
                m_currentLod,
                m_name,
                m_setup,
                m_behaviorGraph,
                m_projectData,
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
                        "nearbyCharacters" => Ok(__Field::m_nearbyCharacters),
                        "currentLod" => Ok(__Field::m_currentLod),
                        "name" => Ok(__Field::m_name),
                        "setup" => Ok(__Field::m_setup),
                        "behaviorGraph" => Ok(__Field::m_behaviorGraph),
                        "projectData" => Ok(__Field::m_projectData),
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
            struct __hkbCharacterVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbCharacter<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbCharacterVisitor<'de> {
                type Value = hkbCharacter<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkbCharacter")
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
                    let mut m_nearbyCharacters: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_currentLod: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_numTracksInLod: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_ragdollDriver: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_characterControllerDriver: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_footIkDriver: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_handIkDriver: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_setup: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_behaviorGraph: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_projectData: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_animationBindingSet: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_raycastInterface: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_world: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_eventQueue: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_worldFromModel: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_poseLocal: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_deleteWorldFromModel: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_deletePoseLocal: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..19usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_nearbyCharacters) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nearbyCharacters",
                                        ),
                                    );
                                }
                                m_nearbyCharacters = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_currentLod) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentLod",
                                        ),
                                    );
                                }
                                m_currentLod = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_numTracksInLod) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numTracksInLod",
                                        ),
                                    );
                                }
                                m_numTracksInLod = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_name) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_ragdollDriver) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ragdollDriver",
                                        ),
                                    );
                                }
                                m_ragdollDriver = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_characterControllerDriver,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterControllerDriver",
                                        ),
                                    );
                                }
                                m_characterControllerDriver = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_footIkDriver) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footIkDriver",
                                        ),
                                    );
                                }
                                m_footIkDriver = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_handIkDriver) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "handIkDriver",
                                        ),
                                    );
                                }
                                m_handIkDriver = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_setup) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("setup"),
                                    );
                                }
                                m_setup = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_behaviorGraph) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "behaviorGraph",
                                        ),
                                    );
                                }
                                m_behaviorGraph = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_projectData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "projectData",
                                        ),
                                    );
                                }
                                m_projectData = _serde::__private::Some(
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
                                    &m_animationBindingSet,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animationBindingSet",
                                        ),
                                    );
                                }
                                m_animationBindingSet = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_raycastInterface) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "raycastInterface",
                                        ),
                                    );
                                }
                                m_raycastInterface = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_world) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("world"),
                                    );
                                }
                                m_world = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(&m_eventQueue) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventQueue",
                                        ),
                                    );
                                }
                                m_eventQueue = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(&m_worldFromModel) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "worldFromModel",
                                        ),
                                    );
                                }
                                m_worldFromModel = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(&m_poseLocal) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "poseLocal",
                                        ),
                                    );
                                }
                                m_poseLocal = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(
                                    &m_deleteWorldFromModel,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deleteWorldFromModel",
                                        ),
                                    );
                                }
                                m_deleteWorldFromModel = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(&m_deletePoseLocal) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deletePoseLocal",
                                        ),
                                    );
                                }
                                m_deletePoseLocal = _serde::__private::Some(
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
                    __A::pad(&mut __map, 2usize, 2usize)?;
                    let m_nearbyCharacters = match m_nearbyCharacters {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nearbyCharacters",
                                ),
                            );
                        }
                    };
                    let m_currentLod = match m_currentLod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentLod",
                                ),
                            );
                        }
                    };
                    let m_numTracksInLod = match m_numTracksInLod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numTracksInLod",
                                ),
                            );
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let m_ragdollDriver = match m_ragdollDriver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ragdollDriver",
                                ),
                            );
                        }
                    };
                    let m_characterControllerDriver = match m_characterControllerDriver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterControllerDriver",
                                ),
                            );
                        }
                    };
                    let m_footIkDriver = match m_footIkDriver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footIkDriver",
                                ),
                            );
                        }
                    };
                    let m_handIkDriver = match m_handIkDriver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "handIkDriver",
                                ),
                            );
                        }
                    };
                    let m_setup = match m_setup {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("setup"),
                            );
                        }
                    };
                    let m_behaviorGraph = match m_behaviorGraph {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "behaviorGraph",
                                ),
                            );
                        }
                    };
                    let m_projectData = match m_projectData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "projectData",
                                ),
                            );
                        }
                    };
                    let m_animationBindingSet = match m_animationBindingSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animationBindingSet",
                                ),
                            );
                        }
                    };
                    let m_raycastInterface = match m_raycastInterface {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "raycastInterface",
                                ),
                            );
                        }
                    };
                    let m_world = match m_world {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("world"),
                            );
                        }
                    };
                    let m_eventQueue = match m_eventQueue {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventQueue",
                                ),
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
                    let m_poseLocal = match m_poseLocal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "poseLocal",
                                ),
                            );
                        }
                    };
                    let m_deleteWorldFromModel = match m_deleteWorldFromModel {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deleteWorldFromModel",
                                ),
                            );
                        }
                    };
                    let m_deletePoseLocal = match m_deletePoseLocal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deletePoseLocal",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbCharacter {
                        __ptr,
                        parent,
                        m_nearbyCharacters,
                        m_currentLod,
                        m_numTracksInLod,
                        m_name,
                        m_ragdollDriver,
                        m_characterControllerDriver,
                        m_footIkDriver,
                        m_handIkDriver,
                        m_setup,
                        m_behaviorGraph,
                        m_projectData,
                        m_animationBindingSet,
                        m_raycastInterface,
                        m_world,
                        m_eventQueue,
                        m_worldFromModel,
                        m_poseLocal,
                        m_deleteWorldFromModel,
                        m_deletePoseLocal,
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
                    let mut m_nearbyCharacters: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_currentLod: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_setup: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_behaviorGraph: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_projectData: _serde::__private::Option<Pointer> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_nearbyCharacters => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_nearbyCharacters) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nearbyCharacters",
                                        ),
                                    );
                                }
                                m_nearbyCharacters = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_currentLod => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_currentLod) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentLod",
                                        ),
                                    );
                                }
                                m_currentLod = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
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
                            __Field::m_setup => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_setup) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("setup"),
                                    );
                                }
                                m_setup = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_behaviorGraph => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_behaviorGraph) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "behaviorGraph",
                                        ),
                                    );
                                }
                                m_behaviorGraph = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_projectData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_projectData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "projectData",
                                        ),
                                    );
                                }
                                m_projectData = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
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
                    let m_nearbyCharacters = match m_nearbyCharacters {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nearbyCharacters",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_currentLod = match m_currentLod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentLod",
                                ),
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
                    let m_setup = match m_setup {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("setup"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_behaviorGraph = match m_behaviorGraph {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "behaviorGraph",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_projectData = match m_projectData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "projectData",
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
                    _serde::__private::Ok(hkbCharacter {
                        __ptr,
                        parent,
                        m_nearbyCharacters,
                        m_currentLod,
                        m_name,
                        m_setup,
                        m_behaviorGraph,
                        m_projectData,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "nearbyCharacters",
                "currentLod",
                "numTracksInLod",
                "name",
                "ragdollDriver",
                "characterControllerDriver",
                "footIkDriver",
                "handIkDriver",
                "setup",
                "behaviorGraph",
                "projectData",
                "animationBindingSet",
                "raycastInterface",
                "world",
                "eventQueue",
                "worldFromModel",
                "poseLocal",
                "deleteWorldFromModel",
                "deletePoseLocal",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCharacter",
                FIELDS,
                __hkbCharacterVisitor {
                    marker: _serde::__private::PhantomData::<hkbCharacter>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
