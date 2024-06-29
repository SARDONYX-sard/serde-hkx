use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleRayCastBatchingManager`
/// -         version: `0`
/// -       signature: `0xed529f13`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleRayCastBatchingManager {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleCastBatchingManager,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpVehicleRayCastBatchingManager {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpVehicleRayCastBatchingManager"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3981614867u32)
        }
    }
    impl __serde::Serialize for hkpVehicleRayCastBatchingManager {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleRayCastBatchingManager", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "registeredVehicles",
                    &self.parent.parent.m_registeredVehicles,
                )?;
            serializer.serialize_field("totalNumWheels", &self.parent.m_totalNumWheels)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "registeredVehicles",
                    &self.parent.parent.m_registeredVehicles,
                )?;
            serializer.end()
        }
    }
};
