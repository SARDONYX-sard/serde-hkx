use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpStorageExtendedMeshShapeMaterial`
/// -         version: `1`
/// -       signature: `0x2ca3e906`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpStorageExtendedMeshShapeMaterial {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpMeshMaterial,
    /// # C++ Info
    /// -          name: `restitution`(ctype: `hkHalf`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_restitution: f16,
    /// # C++ Info
    /// -          name: `friction`(ctype: `hkHalf`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_friction: f16,
    /// # C++ Info
    /// -          name: `userData`(ctype: `hkUlong`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData: u64,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpStorageExtendedMeshShapeMaterial {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpStorageExtendedMeshShapeMaterial"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(748939526u32)
        }
    }
    impl _serde::Serialize for hkpStorageExtendedMeshShapeMaterial {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(748939526u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpStorageExtendedMeshShapeMaterial", class_meta)?;
            serializer.serialize_field("filterInfo", &self.parent.m_filterInfo)?;
            serializer.serialize_field("restitution", &self.m_restitution)?;
            serializer.serialize_field("friction", &self.m_friction)?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.end()
        }
    }
};
