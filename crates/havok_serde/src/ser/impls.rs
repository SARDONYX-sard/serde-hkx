//! Implement [`Serialize`] for each type.
//!
//! By calling serializer inside the type, you can easily serialize the type when you use it.
use crate::lib::*;

use super::{Serialize, Serializer};
use havok_types::*;

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
    ($ty:ty => $ty_wrapper:expr, $fn_name:tt) => {
        impl Serialize for $ty {
            #[inline]
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.$fn_name(&$ty_wrapper(*self))
            }
        }
    };
}

impl_serialize!(*(), serialize_void);
impl_serialize!(*char, serialize_char);
impl_serialize!(*bool, serialize_bool);

impl_serialize!(I8<'_>, serialize_int8);
impl_serialize!(I16<'_>, serialize_int16);
impl_serialize!(I32<'_>, serialize_int32);
impl_serialize!(I64<'_>, serialize_int64);
impl_serialize!(U8<'_>, serialize_uint8);
impl_serialize!(U16<'_>, serialize_uint16);
impl_serialize!(U32<'_>, serialize_uint32);
impl_serialize!(U64<'_>, serialize_uint64);
impl_serialize!( i8 =>  I8::Number, serialize_int8);
impl_serialize!(i16 => I16::Number, serialize_int16);
impl_serialize!(i32 => I32::Number, serialize_int32);
impl_serialize!(i64 => I64::Number, serialize_int64);
impl_serialize!( u8 =>  U8::Number, serialize_uint8);
impl_serialize!(u16 => U16::Number, serialize_uint16);
impl_serialize!(u32 => U32::Number, serialize_uint32);
impl_serialize!(u64 => U64::Number, serialize_uint64);

impl_serialize!(*f16, serialize_half);
impl_serialize!(*f32, serialize_real);

impl_serialize!(Vector4, serialize_vector4);
impl_serialize!(Quaternion, serialize_quaternion);
impl_serialize!(Matrix3, serialize_matrix3);
impl_serialize!(Rotation, serialize_rotation);
impl_serialize!(QsTransform, serialize_qstransform);
impl_serialize!(Matrix4, serialize_matrix4);
impl_serialize!(Transform, serialize_transform);

impl_serialize!(Variant<'_>, serialize_variant);
impl_serialize!(Pointer<'_>, serialize_pointer);
impl_serialize!(*Ulong, serialize_ulong);

impl Serialize for &str {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_stringptr(&StringPtr::from_str(self))
    }
}

impl Serialize for StringPtr<'_> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_stringptr(self)
    }
}

impl Serialize for CString<'_> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_cstring(self)
    }
}

macro_rules! impl_serialize_with_index_array {
    ($($ty:ty),+ $(,)? => $fn_name:tt) => {
        $(
        impl Serialize for Vec<$ty> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use super::SerializeSeq;

                let mut seq = tri!(serializer.serialize_array(Some(self.len())));
                for (index, element) in self.iter().enumerate() {
                    tri!(seq.$fn_name(element, index, self.len()));
                }
                seq.end()
            }
        }

        impl Serialize for &Vec<$ty> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use super::SerializeSeq;

                let mut seq = tri!(serializer.serialize_array(Some(self.len())));
                for (index, element) in self.iter().enumerate() {
                    tri!(seq.$fn_name(element, index, self.len()));
                }
                seq.end()
            }
        }

        impl Serialize for &[$ty] {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use super::SerializeSeq;

                let mut seq = tri!(serializer.serialize_array(Some(self.len())));
                for (index, element) in self.iter().enumerate() {
                    tri!(seq.$fn_name(element, index, self.len()));
                }
                seq.end()
            }
        }
        )*
    };
}

impl_serialize_with_index_array!((), bool, char, U8<'_>, U16<'_>, U32<'_>, U64<'_>, I8<'_>, I16<'_>, I32<'_>, I64<'_>, f16, f32, Pointer<'_>, Ulong => serialize_primitive_element);

macro_rules! impl_serialize_vec {
    ($($ty:ty),+ $(,)? => $fn_name:tt) => {
        $(
        impl Serialize for Vec<$ty> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use super::SerializeSeq;

                let mut seq = tri!(serializer.serialize_array(Some(self.len())));
                for element in self {
                    tri!(seq.$fn_name(element));
                }
                seq.end()
            }
        }

        impl Serialize for &Vec<$ty> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use super::SerializeSeq;

                let mut seq = tri!(serializer.serialize_array(Some(self.len())));
                for element in *self {
                    tri!(seq.$fn_name(element));
                }
                seq.end()
            }
        }

        impl Serialize for &[$ty] {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use super::SerializeSeq;

                let mut seq = tri!(serializer.serialize_array(Some(self.len())));
                for element in *self {
                    tri!(seq.$fn_name(element));
                }
                seq.end()
            }
        }
        )*
    };
}

impl_serialize_vec!(StringPtr<'_> => serialize_stringptr_element);
impl_serialize_vec!(CString<'_> => serialize_cstring_element);
impl_serialize_vec!(Vector4, Quaternion, Matrix3, Rotation, QsTransform, Matrix4, Transform => serialize_math_element);

// impl for Any ClassT.
impl<T: Serialize + crate::HavokClass> Serialize for Vec<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use super::SerializeSeq;

        let mut seq = tri!(serializer.serialize_array(Some(self.len())));
        for element in self {
            tri!(seq.serialize_class_element(element));
        }
        seq.end()
    }
}

// impl for Any ClassT.
impl<T: Serialize + crate::HavokClass> Serialize for &Vec<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use super::SerializeSeq;

        let mut seq = tri!(serializer.serialize_array(Some(self.len())));
        for element in *self {
            tri!(seq.serialize_class_element(element));
        }
        seq.end()
    }
}

impl<T: Serialize + crate::HavokClass> Serialize for &[T] {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use super::SerializeSeq;

        let mut seq = tri!(serializer.serialize_array(Some(self.len())));
        for element in *self {
            tri!(seq.serialize_class_element(element));
        }
        seq.end()
    }
}

#[cfg(feature = "indexmap")]
const _: () = {
    use indexmap::IndexMap;

    impl<K, V> Serialize for &IndexMap<K, V>
    where
        V: Serialize + crate::HavokClass,
    {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use super::SerializeSeq;

            let mut seq = tri!(serializer.serialize_array(Some(self.len())));
            for (_index, element) in self.iter() {
                tri!(seq.serialize_class_element(element));
            }
            seq.end()
        }
    }

    impl<K, V> Serialize for IndexMap<K, V>
    where
        V: Serialize + crate::HavokClass,
    {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use super::SerializeSeq;

            let mut seq = tri!(serializer.serialize_array(Some(self.len())));
            for (_index, element) in self.iter() {
                tri!(seq.serialize_class_element(element));
            }
            seq.end()
        }
    }
};
