#[repr(i32)]
#[doc = " The type of animation interpolation (blending) that will be used on the\n associated key frames."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KeyType {
    // #[doc = "< Use linear interpolation."]
    // Linear = 1,
    // #[doc = "< Use quadratic interpolation.  Forward and back tangents will be stored."]
    // Quadratic = 2,
    // #[doc = "< Use Tension Bias Continuity interpolation.  Tension, bias, and continuity will be stored."]
    // Tbc = 3,
    // #[doc = "< For use only with rotation data.  Separate X, Y, and Z keys will be stored instead of using quaternions."]
    // XyzRotation = 4,
    #[doc = "< Step function. Used for visibility keys in NiBoolData."]
    Const = 5,
}
unsafe impl cxx::ExternType for KeyType {
    type Id = cxx::type_id!("nifbridge::KeyType");
    type Kind = cxx::kind::Trivial;
}

#[doc = "Represents a position or direction in 3D space"]
#[repr(C, align(16))]
#[derive(Debug, Clone)]
pub struct Vector3 {
    #[doc = "< The X component of this vector."]
    pub x: f32,
    #[doc = "< The Y component of this vector."]
    pub y: f32,
    #[doc = "< The Z component of this vector."]
    pub z: f32,
}
impl Vector3 {
    pub const DEFAULT: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}
unsafe impl cxx::ExternType for Vector3 {
    type Id = cxx::type_id!("nifbridge::Vector3");
    type Kind = cxx::kind::Trivial;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Vector3"][::std::mem::size_of::<Vector3>() - 16_usize]; // Niflib size is 12
    ["Alignment of Vector3"][::std::mem::align_of::<Vector3>() - 16_usize]; // Niflib is 4
    ["Offset of field: Vector3::x"][::std::mem::offset_of!(Vector3, x) - 0_usize];
    ["Offset of field: Vector3::y"][::std::mem::offset_of!(Vector3, y) - 4_usize];
    ["Offset of field: Vector3::z"][::std::mem::offset_of!(Vector3, z) - 8_usize];
};

#[repr(C, align(16))]
#[derive(Debug, Clone)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Quaternion {
    pub const DEFAULT: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
}
unsafe impl cxx::ExternType for Quaternion {
    type Id = cxx::type_id!("nifbridge::Quaternion");
    type Kind = cxx::kind::Trivial;
}

#[allow(clippy::panic)]
#[cxx::bridge(namespace = "nifbridge")]
pub(crate) mod ffi {
    #[derive(Debug, Clone)]
    pub struct Vector3Key {
        pub time: f32,
        pub data: Vector3,
    }

    #[derive(Debug, Clone)]
    pub struct QuaternionKey {
        pub time: f32,
        pub data: Quaternion,
    }
    #[derive(Debug, Clone)]
    pub struct FloatKey {
        pub time: f32,
        pub data: f32,
    }

    unsafe extern "C++" {
        include!("wrapper.h");

        // ------------------------------------------------------------
        // Opaque C++ types
        // ------------------------------------------------------------

        type NiControllerSequenceRef;
        type ControllerLink;
        type NiFloatDataRef;
        type NiTransformDataRef;
        type TransformKeys;

        // type FloatKey = crate::kf::to_hkx::bridge::FloatKey;
        // type QuaternionKey = crate::kf::to_hkx::bridge::QuaternionKey;
        // type Vector3Key = crate::kf::to_hkx::bridge::Vector3Key;
        type KeyType = crate::kf::from_kf::bridge::KeyType;
        type Vector3 = crate::kf::from_kf::bridge::Vector3;
        type Quaternion = crate::kf::from_kf::bridge::Quaternion;

        // ------------------------------------------------------------
        // Sequence / controller
        // ------------------------------------------------------------

        fn read_nif_list(name: &CxxString)
        -> Result<UniquePtr<CxxVector<NiControllerSequenceRef>>>;

        fn get_controller_links(
            seq: &NiControllerSequenceRef,
        ) -> UniquePtr<CxxVector<ControllerLink>>;

        fn get_target_name(r: &NiControllerSequenceRef) -> UniquePtr<CxxString>;
        fn get_start_time(r: &NiControllerSequenceRef) -> f32;
        fn get_stop_time(r: &NiControllerSequenceRef) -> f32;

        // ------------------------------------------------------------
        // Transform controller
        // ------------------------------------------------------------

        fn is_transform_type(r: &ControllerLink) -> bool;
        fn get_transform_ref(r: &ControllerLink) -> UniquePtr<NiTransformDataRef>;
        fn get_node_name(r: &ControllerLink) -> &CxxString;

        fn get_transform_keys(r: &NiTransformDataRef) -> UniquePtr<TransformKeys>;

        fn get_translate_key_type(r: &NiTransformDataRef) -> KeyType;
        fn get_rotate_key_type(r: &NiTransformDataRef) -> KeyType;
        fn get_scale_key_type(r: &NiTransformDataRef) -> KeyType;

        // Accessors
        fn get_translate_keys(self: &TransformKeys) -> &CxxVector<Vector3Key>;

        fn get_rotate_keys(self: &TransformKeys) -> &CxxVector<QuaternionKey>;

        fn get_scale_keys(self: &TransformKeys) -> &CxxVector<FloatKey>;

        // ------------------------------------------------------------
        // Float controller
        // ------------------------------------------------------------

        fn is_float_type(r: &ControllerLink) -> bool;
        fn get_float_ref(r: &ControllerLink) -> UniquePtr<NiFloatDataRef>;

        fn get_float_keys(r: &NiFloatDataRef) -> UniquePtr<CxxVector<FloatKey>>;

        fn get_float_key_type(r: &NiFloatDataRef) -> KeyType;
    }
}
