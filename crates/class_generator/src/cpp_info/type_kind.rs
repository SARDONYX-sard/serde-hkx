#![doc = include_str!("../../../../docs/specification/hkx_types.md")]
//! Type kinds used for `vtype` or `vsubtype` in Havok Class.
//!
//! e.g. `hkArray<hkBool>` => `vtype: hkArray, vsubtype: hkBool`
use num_derive::{FromPrimitive, ToPrimitive};
use serde_with::{DeserializeFromStr, SerializeDisplay};

/// Type kinds used in Havok Class.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    SerializeDisplay,
    DeserializeFromStr,
    FromPrimitive,
    ToPrimitive,
)]
pub enum TypeKind {
    /// No type information.
    ///
    /// - C++ type: `void`
    ///
    /// # Examples
    ///
    /// This is often used to fill in generics elements with types for which generics are not used.
    /// - `hkArray<hkBool>` -> `vtype`: `TYPE_ARRAY`, `vsubtype`: `TYPE_BOOL`
    /// - `hkBool` -> `vtype`: `TYPE_BOOL`, `vsubtype`: `TYPE_VOID`
    /// - There is also a pattern `hkArray<void>`. The type information is unknown, but this member always contains the `SERIALIZE_IGNORED` flag and can be skipped.
    #[default]
    Void = 0,

    /// - C++ type: `hkBool` (`bool`)
    Bool,

    /// - C++ type: `hkChar` (`signed char`)
    Char,

    /// - C++ type: `hkInt8` (`signed char`)
    Int8,

    /// - C++ type: `hkUint8` (`unsigned char`)
    Uint8,

    /// - C++ type: `hkInt16` (`signed short`)
    Int16,

    /// - C++ type: `hkUint16` (`unsigned short`)
    Uint16,

    /// - C++ type: `hkInt32` (`signed int`)
    Int32,

    /// - C++ type: `hkUint32` (`unsigned int`)
    Uint32,

    /// - C++ type: `hkInt64` (`signed long long`)
    Int64,

    /// - C++ type: `hkUint64` (`unsigned long long`)
    Uint64,

    /// - C++ type: `hkReal` (`float`)
    Real,

    /// - C++ type: `hkVector4`
    Vector4,

    /// - C++ type: `hkQuaternion`
    Quaternion,

    /// - C++ type: `hkMatrix3`
    Matrix3,

    /// - C++ type: `hkRotation`
    Rotation,

    /// - C++ type: `hkQsTransform`
    QsTransform,

    /// - C++ type: `hkMatrix4`
    Matrix4,

    /// - C++ type: `hkTransform`
    Transform,

    /// Serialize as zero - deprecated.
    ///
    /// # Remarks
    /// Not used in `hk_2010.2.0-r1` havok class.
    Zero,

    /// - C++ type: `T*`
    Pointer,

    /// Function pointer.
    ///
    /// # Remarks
    /// Not used in `hk_2010.2.0-r1` havok class.
    FnPtr,

    /// Array of items of type T.
    /// - C++ type: `hkArray<T>`
    Array,

    /// Array of N items of type T.
    /// - C++ type: `hkInplaceArray<T,N>` or `hkInplaceArrayAligned16<T,N>`
    ///
    /// # Remarks
    /// Not used in `hk_2010.2.0-r1` havok class.
    InplaceArray,

    /// enum type that stores only the size of `SizeType` in memory.
    /// - C++ type: `hkEnum<Enum,SizeType>`
    Enum,

    /// - C++ type: `class` | `struct`
    Struct,

    /// Inline defined pointer and size type.(in `hk_2010.2.0-r1`)
    ///
    /// # Examples
    /// Types used in the five classes.
    /// - `hkbCharacter`(`poseLocal`)
    /// - `hkClass`(`declaredEnums: class hkClassEnum*`, `declaredMembers: class hkClassMember*`)
    /// - `hkClassEnum`(`items`)
    /// - `hkClassMember`(enum item)
    /// - `khkCustomAttributes`(`attributes: struct Attribute*`)
    ///
    /// # Remarks
    /// This can be viewed as a structure consisting of a pointer to a certain class and immediately following it, an `int` representing the number of elements in an array.
    ///
    /// We used the term "viewed as" because this class doesn't actually exist; its fields are directly written into each class.
    SimpleArray,

    /// Simple array of homogeneous types, so is a class id followed by a void* ptr and size
    HomogeneousArray,

    /// - C++ type: `hkVariant` (void* and hkClass*) type
    Variant,

    /// Null terminated string.
    /// - C++ type: `char*`
    CString,

    /// - C++ type: `hkUlong` (`unsigned long`), defined to always be the same size as a pointer
    Ulong,

    /// - C++ type: `hkFlags<ENUM, SizeType>` - 8,16,32 bits of named values.
    Flags,

    /// - C++ type: `hkHalf` (`hkInt16`), 16-bit float value
    Half,

    /// Null-terminated string type.
    ///
    /// There is a flag `StringFlags::OWNED_FLAG = 0x1` defined in the class, so `Owned` is also possible.
    ///
    /// It is unclear which segment (stack, heap, or other) is being pointed to because of the raw pointer.
    /// - C++ type: `hkStringPtr`
    StringPtr,

    /// const array values.
    /// - C++ type: `hkRelArray<T>`
    ///
    /// # Remarks
    /// Not used in `hk_2010.2.0-r1` havok class.
    RelArray,

    /// Max value.
    ///
    /// # Remarks
    /// Not used in `hk_2010.2.0-r1` havok class.
    Max,
}

impl core::fmt::Display for TypeKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TYPE_")?;

        match self {
            Self::Void => write!(f, "VOID"),
            Self::Bool => write!(f, "BOOL"),
            Self::Char => write!(f, "CHAR"),
            Self::Int8 => write!(f, "INT8"),
            Self::Uint8 => write!(f, "UINT8"),
            Self::Int16 => write!(f, "INT16"),
            Self::Uint16 => write!(f, "UINT16"),
            Self::Int32 => write!(f, "INT32"),
            Self::Uint32 => write!(f, "UINT32"),
            Self::Int64 => write!(f, "INT64"),
            Self::Uint64 => write!(f, "UINT64"),
            Self::Real => write!(f, "REAL"),
            Self::Vector4 => write!(f, "VECTOR4"),
            Self::Quaternion => write!(f, "QUATERNION"),
            Self::Matrix3 => write!(f, "MATRIX3"),
            Self::Rotation => write!(f, "ROTATION"),
            Self::QsTransform => write!(f, "QSTRANSFORM"),
            Self::Matrix4 => write!(f, "MATRIX4"),
            Self::Transform => write!(f, "TRANSFORM"),
            Self::Zero => write!(f, "ZERO"),
            Self::Pointer => write!(f, "POINTER"),
            Self::FnPtr => write!(f, "FNPTR"),
            Self::Array => write!(f, "ARRAY"),
            Self::InplaceArray => write!(f, "INPLACEARRAY"),
            Self::Enum => write!(f, "ENUM"),
            Self::Struct => write!(f, "STRUCT"),
            Self::SimpleArray => write!(f, "SIMPLEARRAY"),
            Self::HomogeneousArray => write!(f, "HOMOGENEOUSARRAY"),
            Self::Variant => write!(f, "VARIANT"),
            Self::CString => write!(f, "CSTRING"),
            Self::Ulong => write!(f, "ULONG"),
            Self::Flags => write!(f, "FLAGS"),
            Self::Half => write!(f, "HALF"),
            Self::StringPtr => write!(f, "STRINGPTR"),
            Self::RelArray => write!(f, "RELARRAY"),
            Self::Max => write!(f, "MAX"),
        }
    }
}
impl core::str::FromStr for TypeKind {
    type Err = ParseTypeKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TYPE_VOID" => Ok(Self::Void),
            "TYPE_BOOL" => Ok(Self::Bool),
            "TYPE_CHAR" => Ok(Self::Char),
            "TYPE_INT8" => Ok(Self::Int8),
            "TYPE_UINT8" => Ok(Self::Uint8),
            "TYPE_INT16" => Ok(Self::Int16),
            "TYPE_UINT16" => Ok(Self::Uint16),
            "TYPE_INT32" => Ok(Self::Int32),
            "TYPE_UINT32" => Ok(Self::Uint32),
            "TYPE_INT64" => Ok(Self::Int64),
            "TYPE_UINT64" => Ok(Self::Uint64),
            "TYPE_REAL" => Ok(Self::Real),
            "TYPE_VECTOR4" => Ok(Self::Vector4),
            "TYPE_QUATERNION" => Ok(Self::Quaternion),
            "TYPE_MATRIX3" => Ok(Self::Matrix3),
            "TYPE_ROTATION" => Ok(Self::Rotation),
            "TYPE_QSTRANSFORM" => Ok(Self::QsTransform),
            "TYPE_MATRIX4" => Ok(Self::Matrix4),
            "TYPE_TRANSFORM" => Ok(Self::Transform),
            "TYPE_ZERO" => Ok(Self::Zero),
            "TYPE_POINTER" => Ok(Self::Pointer),
            "TYPE_FNPTR" => Ok(Self::FnPtr),
            "TYPE_ARRAY" => Ok(Self::Array),
            "TYPE_INPLACEARRAY" => Ok(Self::InplaceArray),
            "TYPE_ENUM" => Ok(Self::Enum),
            "TYPE_STRUCT" => Ok(Self::Struct),
            "TYPE_SIMPLEARRAY" => Ok(Self::SimpleArray),
            "TYPE_HOMOGENEOUSARRAY" => Ok(Self::HomogeneousArray),
            "TYPE_VARIANT" => Ok(Self::Variant),
            "TYPE_CSTRING" => Ok(Self::CString),
            "TYPE_ULONG" => Ok(Self::Ulong),
            "TYPE_FLAGS" => Ok(Self::Flags),
            "TYPE_HALF" => Ok(Self::Half),
            "TYPE_STRINGPTR" => Ok(Self::StringPtr),
            "TYPE_RELARRAY" => Ok(Self::RelArray),
            "TYPE_MAX" => Ok(Self::Max),
            _ => Err(ParseTypeKindError),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseTypeKindError;

impl core::fmt::Display for ParseTypeKindError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid TypeKind")
    }
}

impl core::error::Error for ParseTypeKindError {}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn to_string_should_start_with_type_prefix() {
        assert_eq!(TypeKind::Void.to_string(), "TYPE_VOID");
        assert_eq!(TypeKind::Bool.to_string(), "TYPE_BOOL");
        assert_eq!(TypeKind::Char.to_string(), "TYPE_CHAR");
        assert_eq!(TypeKind::StringPtr.to_string(), "TYPE_STRINGPTR");
    }

    #[test]
    fn from_str() {
        assert_eq!("TYPE_BOOL".parse(), Ok(TypeKind::Bool));
        assert_eq!("TYPE_REAL".parse(), Ok(TypeKind::Real));
        assert_eq!("TYPE_QSTRANSFORM".parse(), Ok(TypeKind::QsTransform));
        assert_eq!("TYPE_ARRAY".parse(), Ok(TypeKind::Array));
        assert_eq!("TYPE_STRUCT".parse(), Ok(TypeKind::Struct));
        assert_eq!("TYPE_CSTRING".parse(), Ok(TypeKind::CString));
    }
}
