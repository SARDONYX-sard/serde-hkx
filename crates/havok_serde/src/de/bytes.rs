//! Bytes Deserialization

use crate::error::Result;

/// Deserialize trait for HKX binaries for C++ Havok class.
pub trait ByteDeSerialize<'de>: Sized {
    /// Create a new instance from bytes slice(HKX binary).
    fn from_bytes<D: ByteDeserializer>(deserializer: &'de D, position: &mut u32) -> Result<Self>;
}

pub trait ByteDeserializer {
    fn deserialize_void();
    fn deserialize_bool();
    fn deserialize_char();
    fn deserialize_int8();
    fn deserialize_uint8();
    fn deserialize_int16();
    fn deserialize_uint16();
    fn deserialize_int32();
    fn deserialize_uint32();
    fn deserialize_int64();
    fn deserialize_uint64();
    fn deserialize_real();
    fn deserialize_vector4();
    fn deserialize_quaternion();
    fn deserialize_matrix3();
    fn deserialize_rotation();
    fn deserialize_qstransform();
    fn deserialize_matrix4();
    fn deserialize_transform();
    fn deserialize_zero();
    fn deserialize_pointer();
    fn deserialize_functionpointer();
    fn deserialize_array();
    fn deserialize_inplacearray();
    fn deserialize_enum();
    fn deserialize_struct();
    fn deserialize_simplearray();
    fn deserialize_homogeneousarray();
    fn deserialize_variant();
    fn deserialize_cstring();
    fn deserialize_ulong();
    fn deserialize_flags();
    fn deserialize_half();
    fn deserialize_stringptr();
    fn deserialize_relarray();
    fn deserialize_max();
}
