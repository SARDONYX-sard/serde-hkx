use crate::lib::*;

use havok_serde::de::{self, Visitor};
use havok_serde::Deserialize;
use havok_types::str_parser::{parse_bool, parse_float};
use havok_types::{
    f16, CString, Matrix3, Matrix4, QsTransform, Quaternion, Rotation, StringPtr, Transform,
    Vector4,
};

use crate::de_error::{DeError as Error, Result};

pub struct Deserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    /// from xml string
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

// SERDE IS NOT A PARSING LIBRARY. This impl block defines a few basic parsing
// functions from scratch. More complicated formats may wish to use a dedicated
// parsing library to help implement their Serde deserializer.
impl<'de> Deserializer<'de> {
    // Look at the first character in the input without consuming it.
    fn peek_char(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    // Consume the first character in the input.
    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }

    // Parse a group of decimal digits as an unsigned integer of type T.
    //
    // This implementation is a bit too lenient, for example `001` is not
    // allowed in JSON. Also the various arithmetic operations can overflow and
    // panic or return bogus data. But it is good enough for example code!
    fn parse_unsigned<T>(&mut self) -> Result<T>
    where
        T: std::ops::AddAssign<T> + std::ops::MulAssign<T> + From<u8>,
    {
        let mut int = match self.next_char()? {
            ch @ '0'..='9' => T::from(ch as u8 - b'0'),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };
        loop {
            match self.input.chars().next() {
                Some(ch @ '0'..='9') => {
                    self.input = &self.input[1..];
                    int *= T::from(10);
                    int += T::from(ch as u8 - b'0');
                }
                _ => {
                    return Ok(int);
                }
            }
        }
    }

    // Parse a possible minus sign followed by a group of decimal digits as a
    // signed integer of type T.
    fn parse_signed<T>(&mut self) -> Result<T>
    where
        T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + From<i8>,
    {
        // Optional minus sign, delegate to `parse_unsigned`, negate if negative.
        let mut int = match self.next_char()? {
            ch @ '0'..='9' => T::from((ch as u8 - b'0') as i8),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };
        loop {
            match self.input.chars().next() {
                Some(ch @ '0'..='9') => {
                    self.input = &self.input[1..];
                    int *= T::from(10);
                    int += T::from((ch as u8 - b'0') as i8);
                }
                _ => {
                    return Ok(int);
                }
            }
        }
    }

    // Parse a string until the next '"' character.
    //
    // Makes no attempt to handle escape sequences. What did you expect? This is
    // example code!
    fn parse_string(&mut self) -> Result<&'de str> {
        if self.input.ends_with("<hkcstring>") {
            return Err(Error::ExpectedString);
        }
        match self.input.find("</hkcstring>") {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len + 1..];
                Ok(s)
            }
            None => Err(Error::Eof),
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    #[inline]
    fn deserialize_void<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_void(())
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (remain, boolean) = parse_bool(&self.input)?;
        self.input = remain;
        visitor.visit_bool(boolean)
    }

    #[inline]
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(self.next_char()?)
    }

    #[inline]
    fn deserialize_int8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int8(self.parse_signed()?)
    }

    #[inline]
    fn deserialize_uint8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint8(self.parse_unsigned()?)
    }

    #[inline]
    fn deserialize_int16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int16(self.parse_signed()?)
    }

    #[inline]
    fn deserialize_uint16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint16(self.parse_unsigned()?)
    }

    #[inline]
    fn deserialize_int32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int32(self.parse_signed()?)
    }

    #[inline]
    fn deserialize_uint32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint32(self.parse_unsigned()?)
    }

    #[inline]
    fn deserialize_int64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int64(self.parse_signed()?)
    }

    #[inline]
    fn deserialize_uint64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint64(self.parse_unsigned()?)
    }

    fn deserialize_real<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (remain, float) = parse_float(&self.input)?;
        self.input = remain;
        visitor.visit_real(float)
    }

    fn deserialize_vector4<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (remain, vec) = Vector4::from_str(&self.input)?;
        self.input = remain;
        visitor.visit_vector4(vec)
    }

    fn deserialize_quaternion<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (remain, quaternion) = Quaternion::from_str(&self.input)?;
        self.input = remain;
        visitor.visit_quaternion(quaternion)
    }

    fn deserialize_matrix3<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (remain, matrix) = Matrix3::from_str(&self.input)?;
        self.input = remain;
        visitor.visit_matrix3(matrix)
    }

    fn deserialize_rotation<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (remain, rotation) = Rotation::from_str(&self.input)?;
        self.input = remain;
        visitor.visit_rotation(rotation)
    }

    fn deserialize_qstransform<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (remain, qstransform) = QsTransform::from_str(&self.input)?;
        self.input = remain;
        visitor.visit_qstransform(qstransform)
    }

    fn deserialize_matrix4<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (remain, matrix) = Matrix4::from_str(&self.input)?;
        self.input = remain;
        visitor.visit_matrix4(matrix)
    }

    fn deserialize_transform<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (remain, transform) = Transform::from_str(&self.input)?;
        self.input = remain;
        visitor.visit_transform(transform)
    }

    fn deserialize_pointer<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_array<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_variant<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    #[inline]
    fn deserialize_cstring<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_cstring(CString::from_str(self.parse_string()?))
    }

    #[inline]
    fn deserialize_ulong<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_ulong(self.parse_unsigned()?)
    }

    fn deserialize_enum_flags<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_half<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (remain, float) = parse_float(self.input)?;
        self.input = remain;
        visitor.visit_half(f16::from_f32(float))
    }

    #[inline]
    fn deserialize_stringptr<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_stringptr(StringPtr::from_str(self.parse_string()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::common::mocks::{classes::*, enums::EventMode};
    // use havok_types::*;

    #[test]
    fn test_serialize() {
        // let xml = &include_str!("../../../../docs/handson_hex_dump/defaultmale/defaultmale_x86.xml");
        assert_eq!(from_str::<i32>("32").unwrap(), 32);
    }
}
