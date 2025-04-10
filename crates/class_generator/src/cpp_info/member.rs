//! C++ Class member Information.
use super::{flag_values::FlagValues, type_kind::TypeKind};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// C++ Class member Information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Member<'a> {
    /// Member(Field) name
    #[serde(bound(deserialize = "Cow<'a, str>: Deserialize<'de>"))]
    pub name: Cow<'a, str>,

    /// Used class name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_ref: Option<Cow<'a, str>>,

    /// Used enum name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_ref: Option<Cow<'a, str>>,

    /// Whether `CString` or `StringPtr` is contained in its own member or in a member of its parent?
    /// (To calculate lifetime annotation)
    pub has_string: bool,

    /// C++ Type
    #[serde(bound(deserialize = "Cow<'a, str>: Deserialize<'de>"))]
    pub ctype: Cow<'a, str>,

    /// Havok C++ type kind
    pub vtype: TypeKind,

    /// Type in generics when array, flag or enum member size, pointer of struct, etc. come in.
    pub vsubtype: TypeKind,

    /// Member offset for x86
    pub offset_x86: u32,

    /// Member offset for x86_64
    pub offset_x86_64: u32,

    /// Member size for x86
    pub type_size_x86: u32,

    /// Member size for x86_64
    pub type_size_x86_64: u32,

    /// If an array is used, its size .
    pub arrsize: usize,

    /// Flags for field alignment needs, skipping serialization, etc.
    pub flags: FlagValues,

    /// default member value
    pub default: i32,
}

impl Member<'_> {
    /// Byte size of member.
    ///
    /// Used to calculate the padding of the structure representing the Havok class.
    ///
    /// # Assumptions
    /// Assume that the pointer size is 4 or larger.
    ///
    /// # Errors
    /// - If entered
    ///   `TypeKind::Void | TypeKind::Zero | TypeKind::FnPtr |
    ///  TypeKind::InplaceArray | TypeKind::HomogeneousArray |
    ///  TypeKind::RelArray | TypeKind::Max` => These are not used inside the 2010 Havok Class.
    ///
    /// - `TypeKind::Struct` => This cannot be calculated here. It is calculated externally using `HashMap` or similar.
    pub fn size_of_type(&self, ptr_size: u32) -> Result<u32, MemberError> {
        self.size_of_vtype(ptr_size)
    }

    fn size_of_vtype(&self, ptr_size: u32) -> Result<u32, MemberError> {
        Ok(match self.vtype {
            TypeKind::Enum | TypeKind::Flags => self.size_of_vsubtype(ptr_size)?,

            TypeKind::Void
            | TypeKind::Zero
            | TypeKind::FnPtr
            | TypeKind::InplaceArray
            | TypeKind::HomogeneousArray
            | TypeKind::RelArray
            | TypeKind::Max => {
                return UnsupportedVTypeSnafu {
                    vtype: self.vtype.clone(),
                }
                .fail();
            }

            _ => size_of_type_common(&self.vtype, ptr_size)?,
        })
    }

    fn size_of_vsubtype(&self, ptr_size: u32) -> Result<u32, MemberError> {
        Ok(match self.vsubtype {
            TypeKind::Enum | TypeKind::Flags => {
                return UnsupportedFlagAndEnumOnVSubTypeSnafu.fail();
            }

            TypeKind::Void
            | TypeKind::Zero
            | TypeKind::FnPtr
            | TypeKind::InplaceArray
            | TypeKind::HomogeneousArray
            | TypeKind::RelArray
            | TypeKind::Max => {
                return UnsupportedVSubTypeSnafu {
                    vtype: self.vtype.clone(),
                    vsubtype: self.vsubtype.clone(),
                }
                .fail();
            }

            _ => size_of_type_common(&self.vsubtype, ptr_size)?,
        })
    }

    /// Returns the alignment size of the type.
    ///
    /// This returns information about the internal maximum size needed for offset calculations and tail alignment of structures.
    ///
    /// # Note
    /// Most types are the same as the size TypeKind, but composite types are the size of the inner type.
    ///
    /// - You might think that `Matrix3` and `Vector4` would be 4 bytes because they use f32 internally,
    ///   but in fact they are 16 bytes aligned for SIMD and alignment. `Vector4<f32>` -> 16
    /// - `Variant` -> ptr size
    ///
    /// # Errors
    /// If entered `TypeKind::Void | TypeKind::Zero | TypeKind::FnPtr | TypeKind::InplaceArray | TypeKind::HomogeneousArray | TypeKind::RelArray | TypeKind::Max`.
    ///
    /// These are not used inside the 2010 Havok Class.
    ///
    /// `TypeKind::Struct`: This cannot be calculated here. It is calculated externally using `HashMap` or similar.
    #[allow(clippy::match_same_arms)] // Dare to separate for clarity
    pub fn size_of_align(&self, vtype: &TypeKind, ptr_size: u32) -> Result<u32, MemberError> {
        Ok(match vtype {
            TypeKind::Bool | TypeKind::Char | TypeKind::Int8 | TypeKind::Uint8 => 1,
            TypeKind::Int16 | TypeKind::Uint16 => 2,
            TypeKind::Int32 | TypeKind::Uint32 => 4,
            TypeKind::Int64 | TypeKind::Uint64 => 8,
            TypeKind::Real => 4,

            // Normally, f32 => 4bytes is considered, but in the C++ definition, `align(16)` exists in Vector4, so it needs to be aligned at 16 bytes.
            TypeKind::Vector4
            | TypeKind::Quaternion
            | TypeKind::QsTransform
            | TypeKind::Rotation
            | TypeKind::Matrix3
            | TypeKind::Matrix4
            | TypeKind::Transform => 16,

            TypeKind::Array | TypeKind::SimpleArray => match ptr_size > 4 {
                true => ptr_size, // T* m_data size
                false => 4,       // flag is int
            },
            TypeKind::Struct => return UnsupportedTypeKindStructSnafu.fail(),
            TypeKind::Enum | TypeKind::Flags => self.size_of_type(ptr_size)?,
            TypeKind::Half => 2,

            TypeKind::Pointer
            | TypeKind::Ulong
            | TypeKind::CString
            | TypeKind::StringPtr
            | TypeKind::Variant => ptr_size,

            TypeKind::Void
            | TypeKind::Zero
            | TypeKind::FnPtr
            | TypeKind::InplaceArray
            | TypeKind::HomogeneousArray
            | TypeKind::RelArray
            | TypeKind::Max => {
                return UnsupportedVSubTypeSnafu {
                    vtype: self.vtype.clone(),
                    vsubtype: self.vsubtype.clone(),
                }
                .fail();
            }
        })
    }
}

/// Common processing of vtype and vsubtype.
///
/// The reason for not using recursion is to avoid having to put self.vtype as a borrow argument each time it is called.
/// This makes it easier for the caller.
///
/// # Assumptions
/// Assume that the pointer size is 4 or larger.
///
/// # Errors
/// - If entered `TypeKind::Void(vtype) | TypeKind::Zero | TypeKind::FnPtr |
///    TypeKind::InplaceArray | TypeKind::HomogeneousArray |
///    TypeKind::RelArray | TypeKind::Max` => These are not used inside the 2010 Havok Class.
///
/// - `TypeKind::Struct` => This cannot be calculated here. It is calculated externally using `HashMap` or similar.
#[allow(clippy::match_same_arms)] // Dare to separate for clarity
fn size_of_type_common(type_kind: &TypeKind, ptr_size: u32) -> Result<u32, MemberError> {
    Ok(match type_kind {
        TypeKind::Bool | TypeKind::Char | TypeKind::Int8 | TypeKind::Uint8 => 1,
        TypeKind::Int16 | TypeKind::Uint16 => 2,
        TypeKind::Int32 | TypeKind::Uint32 => 4,
        TypeKind::Int64 | TypeKind::Uint64 => 8,
        TypeKind::Real => 4,

        TypeKind::Vector4 => 16,
        TypeKind::Quaternion => 16, // Vector3<f32>(12) + Scalar<f32>(4) = 16
        TypeKind::QsTransform => 48, // Vector4<f32>(16) + Quaternion<f32>(16) + Vector4<f32>(16)
        TypeKind::Rotation => 48,   // Vector4<f32>(16) * 3
        TypeKind::Matrix3 => 48,    // Vector4<f32>(16) * 3
        TypeKind::Matrix4 => 64,    // Vector4<f32>(16) * 4
        TypeKind::Transform => 64,  // Matrix3<f32>(48) + Vector4<f32>(16)

        TypeKind::Array => match ptr_size {
            4 => 12,
            8 => 16,
            ptr_size => ptr_size + 4 + 4, // ptr_size + size(int) + capAndFlags(int)
        },
        TypeKind::Struct => return UnsupportedTypeKindStructSnafu.fail(),
        TypeKind::SimpleArray => match ptr_size {
            4 => 8,
            8 => 12,
            ptr_size => ptr_size + 4, // ptr + array_len(int)
        },

        TypeKind::Half => 2,
        TypeKind::RelArray => 4, // Not used in havok class of ver. hk2010
        TypeKind::Pointer| TypeKind::FnPtr | TypeKind::Ulong | TypeKind::CString | TypeKind::StringPtr => ptr_size,
        TypeKind::Variant => ptr_size * 2,

        // The following is not common, but vtype and vsubtype have different conditions.
        TypeKind::Enum
        | TypeKind::Flags

        // This is always unsupported.
        | TypeKind::Void
        | TypeKind::Zero
        | TypeKind::InplaceArray
        | TypeKind::HomogeneousArray
        | TypeKind::Max => {
            return UnsupportedVTypeSnafu {
                vtype: type_kind.clone(),
            }
            .fail()
        }
    })
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, snafu::Snafu)]
pub enum MemberError {
    #[snafu(display(
        "Struct cannot calculate the size here. This is obtained by caching members in a HashMap or similar."
    ))]
    UnsupportedTypeKindStruct,

    #[snafu(display(
        "Unsupported vtype: {vtype}. This is because they are not used in the 2010 Havok classes."
    ))]
    UnsupportedVType { vtype: TypeKind },

    #[snafu(display(
        "Unsupported vtype: {vtype}, vsubtype: {vsubtype}. This is because they are not used in the 2010 Havok classes."
    ))]
    UnsupportedVSubType { vtype: TypeKind, vsubtype: TypeKind },

    #[snafu(display(
        "TypeKind::Flag and TypeKind::Enum should not come in generics (vsubtype), so they are not supported (at least in ver. hk2010)."
    ))]
    UnsupportedFlagAndEnumOnVSubType,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_return_correct_type_size_and_align_for_bool() {
        let member = Member {
            vtype: TypeKind::Bool,
            vsubtype: TypeKind::Bool,
            ..Default::default()
        };
        let ptr_size = 4; // Just for testing purposes

        let actual_size = member.size_of_vtype(ptr_size);
        assert_eq!(actual_size, Ok(1));

        let actual_align = member.size_of_align(&member.vtype, ptr_size);
        assert_eq!(actual_align, Ok(1));
    }

    #[test]
    fn should_return_correct_type_size_and_align_for_char() {
        let member = Member {
            vtype: TypeKind::Char,
            vsubtype: TypeKind::Char,
            ..Default::default()
        };
        let ptr_size = 4;
        let actual_size = member.size_of_type(ptr_size);
        let actual_align = member.size_of_align(&member.vtype, ptr_size);
        assert_eq!((actual_size, actual_align), (Ok(1), Ok(1)));
    }
}
