use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleFrictionStatus`
/// -         version: `0`
/// -       signature: `0x1c076a84`
/// -          size:  72(x86)/ 72(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleFrictionStatus {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `axis`(ctype: `struct hkpVehicleFrictionStatusAxisStatus[2]`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  36(x86)/ 72(x86_64)
    ///
    pub m_axis: [hkpVehicleFrictionStatusAxisStatus; 2usize],
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpVehicleFrictionStatus {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleFrictionStatus"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(470248068u32)
        }
    }
    impl __serde::Serialize for hkpVehicleFrictionStatus {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(470248068u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleFrictionStatus", class_meta)?;
            serializer.serialize_field("axis", &self.m_axis.as_slice())?;
            serializer.pad_field([0u8; 36usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.end()
        }
    }
};
