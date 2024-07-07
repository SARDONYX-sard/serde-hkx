use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMultithreadedVehicleManager`
/// -         version: `0`
/// -       signature: `0xc03af40d`
/// -          size:  20(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMultithreadedVehicleManager {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleManager,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpMultithreadedVehicleManager {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMultithreadedVehicleManager"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc03af40d)
        }
    }
    impl _serde::Serialize for hkpMultithreadedVehicleManager {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc03af40d)));
            let mut serializer = __serializer
                .serialize_struct("hkpMultithreadedVehicleManager", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "registeredVehicles",
                    &self.parent.m_registeredVehicles,
                )?;
            serializer
                .serialize_array_field(
                    "registeredVehicles",
                    &self.parent.m_registeredVehicles,
                )?;
            serializer.end()
        }
    }
};
