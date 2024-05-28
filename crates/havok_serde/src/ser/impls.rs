//! Implement [`Serialize`] for each type.
//!
//! By calling serializer inside the type, you can easily serialize the type when you use it.
use crate::lib::*;

use super::{Serialize, Serializer};
use havok_types::{
    f16, Matrix3, Matrix4, Pointer, QsTransform, Quaternion, Rotation, StringPtr, Transform,
    Variant, Vector4,
};

macro_rules! impl_serialize {
    (*$ty:ty, $fn_name:tt) => {
        impl Serialize for $ty {
            #[inline]
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.$fn_name(*self)
            }
        }
    };
    (&$ty:ty, $fn_name:tt) => {
        impl Serialize for $ty {
            #[inline]
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.$fn_name(&self)
            }
        }
    };
    ($ty:ty, $fn_name:tt) => {
        impl Serialize for $ty {
            #[inline]
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.$fn_name(self)
            }
        }
    };
}

impl_serialize!(*char, serialize_char);
impl_serialize!(*bool, serialize_bool);
impl_serialize!(*i8, serialize_int8);
impl_serialize!(*i16, serialize_int16);
impl_serialize!(*i32, serialize_int32);
impl_serialize!(*i64, serialize_int64);
impl_serialize!(*u8, serialize_uint8);
impl_serialize!(*u16, serialize_uint16);
impl_serialize!(*u32, serialize_uint32);
impl_serialize!(*u64, serialize_uint64);
impl_serialize!(*f16, serialize_half);
impl_serialize!(*f32, serialize_real);

impl_serialize!(Vector4, serialize_vector4);
impl_serialize!(Quaternion, serialize_quaternion);
impl_serialize!(Matrix3, serialize_matrix3);
impl_serialize!(Rotation, serialize_rotation);
impl_serialize!(QsTransform, serialize_qstransform);
impl_serialize!(Matrix4, serialize_matrix4);
impl_serialize!(Transform, serialize_transform);

impl_serialize!(Variant, serialize_variant);
impl_serialize!(*Pointer, serialize_pointer);

impl Serialize for &str {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_stringptr(&((*self).into()))
    }
}

impl Serialize for Cow<'_, str> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_stringptr(&self)
    }
}

macro_rules! impl_serialize_primitive_array {
    ($($ty:ty),+ $(,)? => $fn_name:tt) => {
        $(
        impl Serialize for Vec<$ty> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use super::SerializeSeq;

                let mut seq = serializer.serialize_array(Some(self.len()))?;
                for (index, element) in self.iter().enumerate() {
                    seq.$fn_name(element, index, self.len())?;
                }
                seq.end()
            }
        }

        impl Serialize for &Vec<$ty> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use super::SerializeSeq;

                let mut seq = serializer.serialize_array(Some(self.len()))?;
                for (index, element) in self.iter().enumerate() {
                    seq.$fn_name(element, index, self.len())?;
                }
                seq.end()
            }
        }

        impl Serialize for &[$ty] {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use super::SerializeSeq;

                let mut seq = serializer.serialize_array(Some(self.len()))?;
                for (index, element) in self.iter().enumerate() {
                    seq.$fn_name(element, index, self.len())?;
                }
                seq.end()
            }
        }
      )*
    };
}

impl_serialize_primitive_array!(
  bool, char, u8, u16, u32, u64, i8, i16, i32, i64, f32, Pointer
  => serialize_primitive_element
);

macro_rules! impl_serialize_vec {
    ($($ty:ty),+ $(,)? => $fn_name:tt) => {
        $(
        impl Serialize for Vec<$ty> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use super::SerializeSeq;

                let mut seq = serializer.serialize_array(Some(self.len()))?;
                for element in self {
                    seq.$fn_name(element)?;
                }
                seq.end()
            }
        }

        impl Serialize for &Vec<$ty> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use super::SerializeSeq;

                let mut seq = serializer.serialize_array(Some(self.len()))?;
                for element in *self {
                    seq.$fn_name(element)?;
                }
                seq.end()
            }
        }

        impl Serialize for &[$ty] {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use super::SerializeSeq;

                let mut seq = serializer.serialize_array(Some(self.len()))?;
                for element in *self {
                    seq.$fn_name(element)?;
                }
                seq.end()
            }
        }
      )*
    };
}

// impl_serialize_vec!(CString<'_> => serialize_string_element); // Already implemented(By StringPtr).
impl_serialize_vec!(StringPtr<'_> => serialize_string_element);
impl_serialize_vec!(Vector4, Quaternion, Matrix3, Rotation, QsTransform, Matrix4, Transform => serialize_math_element);

// impl for Any ClassT.
impl<T: Serialize + crate::HavokClass> Serialize for Vec<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use super::SerializeSeq;

        let mut seq = serializer.serialize_array(Some(self.len()))?;
        for element in self {
            seq.serialize_class_element(element)?;
        }
        seq.end()
    }
}

// impl for Any ClassT.
impl<T: Serialize + crate::HavokClass> Serialize for &Vec<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use super::SerializeSeq;

        let mut seq = serializer.serialize_array(Some(self.len()))?;
        for element in *self {
            seq.serialize_class_element(element)?;
        }
        seq.end()
    }
}

impl<T: Serialize + crate::HavokClass> Serialize for &[T] {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use super::SerializeSeq;

        let mut seq = serializer.serialize_array(Some(self.len()))?;
        for element in *self {
            seq.serialize_class_element(element)?;
        }
        seq.end()
    }
}
