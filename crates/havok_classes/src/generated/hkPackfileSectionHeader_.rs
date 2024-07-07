use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkPackfileSectionHeader`
/// -         version: `0`
/// -       signature: `0xf2a92154`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkPackfileSectionHeader {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `sectionTag`(ctype: `hkChar[19]`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  19(x86)/ 19(x86_64)
    ///
    pub m_sectionTag: [char; 19usize],
    /// # C++ Info
    /// -          name: `nullByte`(ctype: `hkChar`)
    /// -        offset:  19(x86)/ 19(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_nullByte: char,
    /// # C++ Info
    /// -          name: `absoluteDataStart`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_absoluteDataStart: i32,
    /// # C++ Info
    /// -          name: `localFixupsOffset`(ctype: `hkInt32`)
    /// -        offset:  24(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_localFixupsOffset: i32,
    /// # C++ Info
    /// -          name: `globalFixupsOffset`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_globalFixupsOffset: i32,
    /// # C++ Info
    /// -          name: `virtualFixupsOffset`(ctype: `hkInt32`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_virtualFixupsOffset: i32,
    /// # C++ Info
    /// -          name: `exportsOffset`(ctype: `hkInt32`)
    /// -        offset:  36(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_exportsOffset: i32,
    /// # C++ Info
    /// -          name: `importsOffset`(ctype: `hkInt32`)
    /// -        offset:  40(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_importsOffset: i32,
    /// # C++ Info
    /// -          name: `endOffset`(ctype: `hkInt32`)
    /// -        offset:  44(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_endOffset: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkPackfileSectionHeader {
        #[inline]
        fn name(&self) -> &'static str {
            "hkPackfileSectionHeader"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf2a92154)
        }
    }
    impl _serde::Serialize for hkPackfileSectionHeader {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf2a92154)));
            let mut serializer = __serializer
                .serialize_struct("hkPackfileSectionHeader", class_meta)?;
            serializer.serialize_field("sectionTag", &self.m_sectionTag.as_slice())?;
            serializer.serialize_field("nullByte", &self.m_nullByte)?;
            serializer.serialize_field("absoluteDataStart", &self.m_absoluteDataStart)?;
            serializer.serialize_field("localFixupsOffset", &self.m_localFixupsOffset)?;
            serializer
                .serialize_field("globalFixupsOffset", &self.m_globalFixupsOffset)?;
            serializer
                .serialize_field("virtualFixupsOffset", &self.m_virtualFixupsOffset)?;
            serializer.serialize_field("exportsOffset", &self.m_exportsOffset)?;
            serializer.serialize_field("importsOffset", &self.m_importsOffset)?;
            serializer.serialize_field("endOffset", &self.m_endOffset)?;
            serializer.end()
        }
    }
};
