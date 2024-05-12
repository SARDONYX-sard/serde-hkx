#![doc = include_str!("../../../../docs/specification/hkx_types.md")]
//! Type kinds used for `vtype` or `vsubtype` in Havok Class.
use havok_types::impl_str_serde;
use num_derive::{FromPrimitive, ToPrimitive};
use parse_display::{Display, FromStr};

impl_str_serde!(TypeKind);

/// Type kinds used in Havok Class.
#[derive(Debug, Clone, Default, PartialEq, Eq, FromStr, Display, FromPrimitive, ToPrimitive)]
#[display("TYPE_{}", style = "UPPERCASE")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_cast_to_string() {
        assert_eq!(TypeKind::Void.to_string(), "TYPE_VOID");
        assert_eq!(TypeKind::Bool.to_string(), "TYPE_BOOL");
        assert_eq!(TypeKind::Char.to_string(), "TYPE_CHAR");
        assert_eq!(TypeKind::StringPtr.to_string(), "TYPE_STRINGPTR");
    }

    #[test]
    fn should_cast_from_str() {
        assert_eq!("TYPE_BOOL".parse(), Ok(TypeKind::Bool));
        assert_eq!("TYPE_REAL".parse(), Ok(TypeKind::Real));
        assert_eq!("TYPE_QSTRANSFORM".parse(), Ok(TypeKind::QsTransform));
        assert_eq!("TYPE_ARRAY".parse(), Ok(TypeKind::Array));
        assert_eq!("TYPE_STRUCT".parse(), Ok(TypeKind::Struct));
        assert_eq!("TYPE_CSTRING".parse(), Ok(TypeKind::CString));
    }
}
