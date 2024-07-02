use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDefaultSuspension`
/// -         version: `0`
/// -       signature: `0x21735a24`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultSuspension {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleSuspension,
    /// # C++ Info
    /// -          name: `wheelSpringParams`(ctype: `hkArray<struct hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wheelSpringParams: Vec<
        hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters,
    >,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpVehicleDefaultSuspension {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultSuspension"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(561207844u32)
        }
    }
    impl __serde::Serialize for hkpVehicleDefaultSuspension {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(561207844u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDefaultSuspension", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("wheelParams", &self.parent.m_wheelParams)?;
            serializer
                .serialize_array_meta_field(
                    "wheelSpringParams",
                    &self.m_wheelSpringParams,
                )?;
            serializer.serialize_array_field("wheelParams", &self.parent.m_wheelParams)?;
            serializer
                .serialize_array_field("wheelSpringParams", &self.m_wheelSpringParams)?;
            serializer.end()
        }
    }
};
