use super::class::*;
use super::HkReferencedObject;

/// `hkbProjectStringData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 76
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x76ad60a`
/// -   version: 1
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbProjectStringData<'a> {
    pub parent: HkReferencedObject,

    pub _name: Option<Pointer>,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"animationFilenames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub animation_filenames: Vec<StringPtr<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"behaviorFilenames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub behavior_filenames: Vec<StringPtr<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"characterFilenames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub character_filenames: Vec<StringPtr<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"eventNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub event_names: Vec<StringPtr<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"animationPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub animation_path: StringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"behaviorPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub behavior_path: StringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"characterPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub character_path: StringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"fullPathToSource"`
    /// -   type: `hkStringPtr`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub full_path_to_source: StringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"rootPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub root_path: StringPtr<'a>,
}

impl HavokClass for HkbProjectStringData<'_> {}
impl Serialize for HkbProjectStringData<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use havok_serde::ser::SerializeStruct;

        let class_meta = self._name.map(|name| (name, Signature::new(0xea7f1d08)));
        let mut serializer = serializer.serialize_struct("hkbProjectStringData", class_meta)?;

        serializer.serialize_array_field("animationFilenames", &self.animation_filenames)?;
        serializer.serialize_array_field("behaviorFilenames", &self.behavior_filenames)?;
        serializer.serialize_array_field("characterFilenames", &self.character_filenames)?;
        serializer.serialize_array_field("eventNames", &self.event_names)?;
        serializer.serialize_field("animationPath", &self.animation_path)?;
        serializer.serialize_field("behaviorPath", &self.behavior_path)?;
        serializer.serialize_field("characterPath", &self.character_path)?;
        serializer.serialize_field("fullPathToSource", &self.full_path_to_source)?;
        serializer.skip_field("rootPath", &self.root_path)?;
        serializer.end()
    }
}
