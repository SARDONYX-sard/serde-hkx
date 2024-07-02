use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleFrictionDescription`
/// -         version: `0`
/// -       signature: `0x1034549a`
/// -          size: 208(x86)/208(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleFrictionDescription {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `wheelDistance`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelDistance: f32,
    /// # C++ Info
    /// -          name: `chassisMassInv`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_chassisMassInv: f32,
    /// # C++ Info
    /// -          name: `axleDescr`(ctype: `struct hkpVehicleFrictionDescriptionAxisDescription[2]`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size: 100(x86)/200(x86_64)
    ///
    pub m_axleDescr: [hkpVehicleFrictionDescriptionAxisDescription; 2usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleFrictionDescription {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleFrictionDescription"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(271864986u32)
        }
    }
    impl _serde::Serialize for hkpVehicleFrictionDescription {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(271864986u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleFrictionDescription", class_meta)?;
            serializer.serialize_field("wheelDistance", &self.m_wheelDistance)?;
            serializer.serialize_field("chassisMassInv", &self.m_chassisMassInv)?;
            serializer.serialize_field("axleDescr", &self.m_axleDescr.as_slice())?;
            serializer.pad_field([0u8; 100usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.end()
        }
    }
};
