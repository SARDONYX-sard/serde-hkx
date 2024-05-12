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

    /// Member offset for x86
    pub offset_x86: u32,

    /// Member offset for x86_64
    pub offset_x86_64: u32,

    /// Used class name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_ref: Option<Cow<'a, str>>,

    /// Used enum name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_ref: Option<Cow<'a, str>>,

    /// C++ Type
    #[serde(bound(deserialize = "Cow<'a, str>: Deserialize<'de>"))]
    pub ctype: Cow<'a, str>,

    /// Havok Type enumeration (Rough category of `Self::type_name`.)
    pub vtype: TypeKind,

    /// Type in generics when arrays, etc. come in.
    pub vsubtype: TypeKind,

    /// If an array is used, its size .
    pub arrsize: usize,

    /// Flags for field alignment needs, skipping serialization, etc.
    pub flags: FlagValues,

    /// default member value
    pub default: i32,
}

impl Member<'_> {
    /// Byte size that must be read. Used to calculate the padding of the structure representing the Havok class.
    ///
    /// # Assumptions
    /// Assume that the pointer size is 4 or larger.
    ///
    /// # Panics
    /// - If entered
    /// `TypeKind::Void | TypeKind::Zero | TypeKind::FnPtr |
    ///  TypeKind::InplaceArray | TypeKind::HomogeneousArray |
    ///  TypeKind::RelArray | TypeKind::Max` => These are not used inside the 2010 Havok Class.
    ///
    /// - `TypeKind::Array` & ptr_size > 8 => Unsupported
    /// - `TypeKind::Struct` => This cannot be calculated here. It is calculated externally using `HashMap` or similar.
    pub fn type_size(&self, vtype: &TypeKind, ptr_size: u32) -> u32 {
        match vtype {
            TypeKind::Bool => 1,
            TypeKind::Char => 1,
            TypeKind::Int8 => 1,
            TypeKind::Uint8 => 1,
            TypeKind::Int16 => 2,
            TypeKind::Uint16 => 2,
            TypeKind::Int32 => 4,
            TypeKind::Uint32 => 4,
            TypeKind::Int64 => 8,
            TypeKind::Uint64 => 8,
            TypeKind::Real => 4,

            TypeKind::Vector4 => 16,
            TypeKind::Quaternion => 16, // Vector3<f32>(12) + Scalar<f32>(4) = 16
            TypeKind::QsTransform => 48, // Vector4<f32>(16) + Quaternion<f32>(16) + Vector4<f32>(16)
            TypeKind::Rotation => 48,    // Vector4<f32>(12) * 3
            TypeKind::Matrix3 => 48,     // Vector4<f32> * 3
            TypeKind::Matrix4 => 64,     // Vector4<f32>(16) * 4
            TypeKind::Transform => 64,   // Matrix3<f32>(48) + Vector4<f32>(16)

            TypeKind::Array => match ptr_size {
                4 => 12,
                8 => 16,
                ptr_size => ptr_size + 4 + 4, // ptr + size(int) + capAndFlags(int)
            },
            TypeKind::Struct => panic!("Struct cannot calculate the size here. This is obtained by caching members in a HashMap or similar."),
            TypeKind::SimpleArray => match ptr_size {
                4 => 8,
                8 => 12,
                ptr_size => ptr_size + 4, // ptr + array_len(int)
            },
            TypeKind::Enum | TypeKind::Flags => self.type_size(&self.vsubtype, ptr_size),
            TypeKind::Half => 2,

            TypeKind::Pointer | TypeKind::Ulong | TypeKind::CString | TypeKind::StringPtr => {
                ptr_size
            }
            TypeKind::Variant => ptr_size * 2,

            TypeKind::Void
            | TypeKind::Zero
            | TypeKind::FnPtr
            | TypeKind::InplaceArray
            | TypeKind::HomogeneousArray
            | TypeKind::RelArray
            | TypeKind::Max => unimplemented!(
                "Unsupported vtype: {:?}, vsubtype: {:?}",
                self.vtype,
                self.vsubtype
            ),
        }
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
    /// # Panics
    /// If entered `TypeKind::Void | TypeKind::Zero | TypeKind::FnPtr | TypeKind::InplaceArray | TypeKind::HomogeneousArray | TypeKind::RelArray | TypeKind::Max`.
    ///
    /// These are not used inside the 2010 Havok Class.
    ///
    /// `TypeKind::Struct`: This cannot be calculated here. It is calculated externally using `HashMap` or similar.
    pub fn size_of_align(&self, vtype: &TypeKind, ptr_size: u32) -> u32 {
        match vtype {
            TypeKind::Bool => 1,
            TypeKind::Char => 1,
            TypeKind::Int8 => 1,
            TypeKind::Uint8 => 1,
            TypeKind::Int16 => 2,
            TypeKind::Uint16 => 2,
            TypeKind::Int32 => 4,
            TypeKind::Uint32 => 4,
            TypeKind::Int64 => 8,
            TypeKind::Uint64 => 8,
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
            TypeKind::Struct => panic!("Unsupported Struct. use cache by HashMap"),
            TypeKind::Enum | TypeKind::Flags => self.type_size(&self.vsubtype, ptr_size),
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
                unimplemented!(
                    "Unsupported vtype: {:?}, vsubtype: {:?}",
                    self.vtype,
                    self.vsubtype
                )
            }
        }
    }
}
