use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkaFootstepAnalysisInfo`
/// - version: `1`
/// - signature: `0x824faf75`
/// - size: `152`(x86)/`208`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaFootstepAnalysisInfo {
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
    /// - name: `name`(ctype: `hkArray<hkChar>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_name: Vec<char>,
    /// # C++ Info
    /// - name: `nameStrike`(ctype: `hkArray<hkChar>`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_nameStrike: Vec<char>,
    /// # C++ Info
    /// - name: `nameLift`(ctype: `hkArray<hkChar>`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_nameLift: Vec<char>,
    /// # C++ Info
    /// - name: `nameLock`(ctype: `hkArray<hkChar>`)
    /// - offset: ` 44`(x86)/` 64`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_nameLock: Vec<char>,
    /// # C++ Info
    /// - name: `nameUnlock`(ctype: `hkArray<hkChar>`)
    /// - offset: ` 56`(x86)/` 80`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_nameUnlock: Vec<char>,
    /// # C++ Info
    /// - name: `minPos`(ctype: `hkArray<hkReal>`)
    /// - offset: ` 68`(x86)/` 96`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_minPos: Vec<f32>,
    /// # C++ Info
    /// - name: `maxPos`(ctype: `hkArray<hkReal>`)
    /// - offset: ` 80`(x86)/`112`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_maxPos: Vec<f32>,
    /// # C++ Info
    /// - name: `minVel`(ctype: `hkArray<hkReal>`)
    /// - offset: ` 92`(x86)/`128`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_minVel: Vec<f32>,
    /// # C++ Info
    /// - name: `maxVel`(ctype: `hkArray<hkReal>`)
    /// - offset: `104`(x86)/`144`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_maxVel: Vec<f32>,
    /// # C++ Info
    /// - name: `allBonesDown`(ctype: `hkArray<hkReal>`)
    /// - offset: `116`(x86)/`160`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_allBonesDown: Vec<f32>,
    /// # C++ Info
    /// - name: `anyBonesDown`(ctype: `hkArray<hkReal>`)
    /// - offset: `128`(x86)/`176`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_anyBonesDown: Vec<f32>,
    /// # C++ Info
    /// - name: `posTol`(ctype: `hkReal`)
    /// - offset: `140`(x86)/`192`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_posTol: f32,
    /// # C++ Info
    /// - name: `velTol`(ctype: `hkReal`)
    /// - offset: `144`(x86)/`196`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_velTol: f32,
    /// # C++ Info
    /// - name: `duration`(ctype: `hkReal`)
    /// - offset: `148`(x86)/`200`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_duration: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaFootstepAnalysisInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaFootstepAnalysisInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x824faf75)
        }
    }
    impl _serde::Serialize for hkaFootstepAnalysisInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x824faf75)));
            let mut serializer = __serializer
                .serialize_struct("hkaFootstepAnalysisInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("name", &self.m_name)?;
            serializer.serialize_array_meta_field("nameStrike", &self.m_nameStrike)?;
            serializer.serialize_array_meta_field("nameLift", &self.m_nameLift)?;
            serializer.serialize_array_meta_field("nameLock", &self.m_nameLock)?;
            serializer.serialize_array_meta_field("nameUnlock", &self.m_nameUnlock)?;
            serializer.serialize_array_meta_field("minPos", &self.m_minPos)?;
            serializer.serialize_array_meta_field("maxPos", &self.m_maxPos)?;
            serializer.serialize_array_meta_field("minVel", &self.m_minVel)?;
            serializer.serialize_array_meta_field("maxVel", &self.m_maxVel)?;
            serializer.serialize_array_meta_field("allBonesDown", &self.m_allBonesDown)?;
            serializer.serialize_array_meta_field("anyBonesDown", &self.m_anyBonesDown)?;
            serializer.serialize_field("posTol", &self.m_posTol)?;
            serializer.serialize_field("velTol", &self.m_velTol)?;
            serializer.serialize_field("duration", &self.m_duration)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_field("name", &self.m_name)?;
            serializer.serialize_array_field("nameStrike", &self.m_nameStrike)?;
            serializer.serialize_array_field("nameLift", &self.m_nameLift)?;
            serializer.serialize_array_field("nameLock", &self.m_nameLock)?;
            serializer.serialize_array_field("nameUnlock", &self.m_nameUnlock)?;
            serializer.serialize_array_field("minPos", &self.m_minPos)?;
            serializer.serialize_array_field("maxPos", &self.m_maxPos)?;
            serializer.serialize_array_field("minVel", &self.m_minVel)?;
            serializer.serialize_array_field("maxVel", &self.m_maxVel)?;
            serializer.serialize_array_field("allBonesDown", &self.m_allBonesDown)?;
            serializer.serialize_array_field("anyBonesDown", &self.m_anyBonesDown)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_name,
    m_nameStrike,
    m_nameLift,
    m_nameLock,
    m_nameUnlock,
    m_minPos,
    m_maxPos,
    m_minVel,
    m_maxVel,
    m_allBonesDown,
    m_anyBonesDown,
    m_posTol,
    m_velTol,
    m_duration,
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
            "name" => Ok(__Field::m_name),
            "nameStrike" => Ok(__Field::m_nameStrike),
            "nameLift" => Ok(__Field::m_nameLift),
            "nameLock" => Ok(__Field::m_nameLock),
            "nameUnlock" => Ok(__Field::m_nameUnlock),
            "minPos" => Ok(__Field::m_minPos),
            "maxPos" => Ok(__Field::m_maxPos),
            "minVel" => Ok(__Field::m_minVel),
            "maxVel" => Ok(__Field::m_maxVel),
            "allBonesDown" => Ok(__Field::m_allBonesDown),
            "anyBonesDown" => Ok(__Field::m_anyBonesDown),
            "posTol" => Ok(__Field::m_posTol),
            "velTol" => Ok(__Field::m_velTol),
            "duration" => Ok(__Field::m_duration),
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
pub(super) struct __hkaFootstepAnalysisInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkaFootstepAnalysisInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkaFootstepAnalysisInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkaFootstepAnalysisInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkaFootstepAnalysisInfo>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkaFootstepAnalysisInfoVisitor<'de> {
    type Value = hkaFootstepAnalysisInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkaFootstepAnalysisInfo")
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
        let mut m_name: _serde::__private::Option<Vec<char>> = _serde::__private::None;
        let mut m_nameStrike: _serde::__private::Option<Vec<char>> = _serde::__private::None;
        let mut m_nameLift: _serde::__private::Option<Vec<char>> = _serde::__private::None;
        let mut m_nameLock: _serde::__private::Option<Vec<char>> = _serde::__private::None;
        let mut m_nameUnlock: _serde::__private::Option<Vec<char>> = _serde::__private::None;
        let mut m_minPos: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_maxPos: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_minVel: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_maxVel: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_allBonesDown: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_anyBonesDown: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_posTol: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_velTol: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_duration: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..14usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_name) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                        );
                    }
                    m_name = _serde::__private::Some(
                        match __A::next_value::<Vec<char>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_nameStrike) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nameStrike",
                            ),
                        );
                    }
                    m_nameStrike = _serde::__private::Some(
                        match __A::next_value::<Vec<char>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_nameLift) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nameLift",
                            ),
                        );
                    }
                    m_nameLift = _serde::__private::Some(
                        match __A::next_value::<Vec<char>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_nameLock) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nameLock",
                            ),
                        );
                    }
                    m_nameLock = _serde::__private::Some(
                        match __A::next_value::<Vec<char>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_nameUnlock) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nameUnlock",
                            ),
                        );
                    }
                    m_nameUnlock = _serde::__private::Some(
                        match __A::next_value::<Vec<char>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_minPos) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("minPos"),
                        );
                    }
                    m_minPos = _serde::__private::Some(
                        match __A::next_value::<Vec<f32>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_maxPos) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("maxPos"),
                        );
                    }
                    m_maxPos = _serde::__private::Some(
                        match __A::next_value::<Vec<f32>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_minVel) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("minVel"),
                        );
                    }
                    m_minVel = _serde::__private::Some(
                        match __A::next_value::<Vec<f32>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_maxVel) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("maxVel"),
                        );
                    }
                    m_maxVel = _serde::__private::Some(
                        match __A::next_value::<Vec<f32>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_allBonesDown) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "allBonesDown",
                            ),
                        );
                    }
                    m_allBonesDown = _serde::__private::Some(
                        match __A::next_value::<Vec<f32>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_anyBonesDown) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "anyBonesDown",
                            ),
                        );
                    }
                    m_anyBonesDown = _serde::__private::Some(
                        match __A::next_value::<Vec<f32>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_posTol) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("posTol"),
                        );
                    }
                    m_posTol = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_velTol) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("velTol"),
                        );
                    }
                    m_velTol = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
                    if _serde::__private::Option::is_some(&m_duration) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "duration",
                            ),
                        );
                    }
                    m_duration = _serde::__private::Some(
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
        __A::pad(&mut __map, 0usize, 4usize)?;
        let m_name = match m_name {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("name"),
                );
            }
        };
        let m_nameStrike = match m_nameStrike {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nameStrike"),
                );
            }
        };
        let m_nameLift = match m_nameLift {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nameLift"),
                );
            }
        };
        let m_nameLock = match m_nameLock {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nameLock"),
                );
            }
        };
        let m_nameUnlock = match m_nameUnlock {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nameUnlock"),
                );
            }
        };
        let m_minPos = match m_minPos {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("minPos"),
                );
            }
        };
        let m_maxPos = match m_maxPos {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxPos"),
                );
            }
        };
        let m_minVel = match m_minVel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("minVel"),
                );
            }
        };
        let m_maxVel = match m_maxVel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxVel"),
                );
            }
        };
        let m_allBonesDown = match m_allBonesDown {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("allBonesDown"),
                );
            }
        };
        let m_anyBonesDown = match m_anyBonesDown {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("anyBonesDown"),
                );
            }
        };
        let m_posTol = match m_posTol {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("posTol"),
                );
            }
        };
        let m_velTol = match m_velTol {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("velTol"),
                );
            }
        };
        let m_duration = match m_duration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("duration"),
                );
            }
        };
        _serde::__private::Ok(hkaFootstepAnalysisInfo {
            __ptr,
            parent,
            m_name,
            m_nameStrike,
            m_nameLift,
            m_nameLock,
            m_nameUnlock,
            m_minPos,
            m_maxPos,
            m_minVel,
            m_maxVel,
            m_allBonesDown,
            m_anyBonesDown,
            m_posTol,
            m_velTol,
            m_duration,
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
        let mut m_name: _serde::__private::Option<Vec<char>> = _serde::__private::None;
        let mut m_nameStrike: _serde::__private::Option<Vec<char>> = _serde::__private::None;
        let mut m_nameLift: _serde::__private::Option<Vec<char>> = _serde::__private::None;
        let mut m_nameLock: _serde::__private::Option<Vec<char>> = _serde::__private::None;
        let mut m_nameUnlock: _serde::__private::Option<Vec<char>> = _serde::__private::None;
        let mut m_minPos: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_maxPos: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_minVel: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_maxVel: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_allBonesDown: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_anyBonesDown: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_posTol: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_velTol: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_duration: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..14usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_name => {
                        if _serde::__private::Option::is_some(&m_name) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                            );
                        }
                        m_name = _serde::__private::Some(
                            match __A::next_value::<Vec<char>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_nameStrike => {
                        if _serde::__private::Option::is_some(&m_nameStrike) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "nameStrike",
                                ),
                            );
                        }
                        m_nameStrike = _serde::__private::Some(
                            match __A::next_value::<Vec<char>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_nameLift => {
                        if _serde::__private::Option::is_some(&m_nameLift) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "nameLift",
                                ),
                            );
                        }
                        m_nameLift = _serde::__private::Some(
                            match __A::next_value::<Vec<char>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_nameLock => {
                        if _serde::__private::Option::is_some(&m_nameLock) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "nameLock",
                                ),
                            );
                        }
                        m_nameLock = _serde::__private::Some(
                            match __A::next_value::<Vec<char>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_nameUnlock => {
                        if _serde::__private::Option::is_some(&m_nameUnlock) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "nameUnlock",
                                ),
                            );
                        }
                        m_nameUnlock = _serde::__private::Some(
                            match __A::next_value::<Vec<char>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_minPos => {
                        if _serde::__private::Option::is_some(&m_minPos) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("minPos"),
                            );
                        }
                        m_minPos = _serde::__private::Some(
                            match __A::next_value::<Vec<f32>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxPos => {
                        if _serde::__private::Option::is_some(&m_maxPos) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("maxPos"),
                            );
                        }
                        m_maxPos = _serde::__private::Some(
                            match __A::next_value::<Vec<f32>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_minVel => {
                        if _serde::__private::Option::is_some(&m_minVel) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("minVel"),
                            );
                        }
                        m_minVel = _serde::__private::Some(
                            match __A::next_value::<Vec<f32>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxVel => {
                        if _serde::__private::Option::is_some(&m_maxVel) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("maxVel"),
                            );
                        }
                        m_maxVel = _serde::__private::Some(
                            match __A::next_value::<Vec<f32>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_allBonesDown => {
                        if _serde::__private::Option::is_some(&m_allBonesDown) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "allBonesDown",
                                ),
                            );
                        }
                        m_allBonesDown = _serde::__private::Some(
                            match __A::next_value::<Vec<f32>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_anyBonesDown => {
                        if _serde::__private::Option::is_some(&m_anyBonesDown) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "anyBonesDown",
                                ),
                            );
                        }
                        m_anyBonesDown = _serde::__private::Some(
                            match __A::next_value::<Vec<f32>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_posTol => {
                        if _serde::__private::Option::is_some(&m_posTol) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("posTol"),
                            );
                        }
                        m_posTol = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_velTol => {
                        if _serde::__private::Option::is_some(&m_velTol) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("velTol"),
                            );
                        }
                        m_velTol = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_duration => {
                        if _serde::__private::Option::is_some(&m_duration) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "duration",
                                ),
                            );
                        }
                        m_duration = _serde::__private::Some(
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
        }
        let m_name = match m_name {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("name"),
                );
            }
        };
        let m_nameStrike = match m_nameStrike {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nameStrike"),
                );
            }
        };
        let m_nameLift = match m_nameLift {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nameLift"),
                );
            }
        };
        let m_nameLock = match m_nameLock {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nameLock"),
                );
            }
        };
        let m_nameUnlock = match m_nameUnlock {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nameUnlock"),
                );
            }
        };
        let m_minPos = match m_minPos {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("minPos"),
                );
            }
        };
        let m_maxPos = match m_maxPos {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxPos"),
                );
            }
        };
        let m_minVel = match m_minVel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("minVel"),
                );
            }
        };
        let m_maxVel = match m_maxVel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxVel"),
                );
            }
        };
        let m_allBonesDown = match m_allBonesDown {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("allBonesDown"),
                );
            }
        };
        let m_anyBonesDown = match m_anyBonesDown {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("anyBonesDown"),
                );
            }
        };
        let m_posTol = match m_posTol {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("posTol"),
                );
            }
        };
        let m_velTol = match m_velTol {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("velTol"),
                );
            }
        };
        let m_duration = match m_duration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("duration"),
                );
            }
        };
        _serde::__private::Ok(hkaFootstepAnalysisInfo {
            __ptr,
            parent,
            m_name,
            m_nameStrike,
            m_nameLift,
            m_nameLock,
            m_nameUnlock,
            m_minPos,
            m_maxPos,
            m_minVel,
            m_maxVel,
            m_allBonesDown,
            m_anyBonesDown,
            m_posTol,
            m_velTol,
            m_duration,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkaFootstepAnalysisInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "name",
                "nameStrike",
                "nameLift",
                "nameLock",
                "nameUnlock",
                "minPos",
                "maxPos",
                "minVel",
                "maxVel",
                "allBonesDown",
                "anyBonesDown",
                "posTol",
                "velTol",
                "duration",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaFootstepAnalysisInfo",
                FIELDS,
                __hkaFootstepAnalysisInfoVisitor {
                    marker: _serde::__private::PhantomData::<hkaFootstepAnalysisInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
