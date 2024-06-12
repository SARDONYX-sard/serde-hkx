//! XML Serialization
use crate::lib::*;

use crate::error::{Error, Result};
use havok_serde::ser::{Serialize, SerializeFlags, SerializeStruct};
use havok_types::variant::Variant;
use havok_types::{
    f16, CString, Matrix3, Matrix4, Pointer, QsTransform, Quaternion, Rotation, Signature,
    StringPtr, Transform, Vector4,
};

#[derive(Debug)]
struct XmlSerializer {
    output: String,
    indent: &'static str,
    depth: usize,
    start_root: Option<&'static str>,
    end_root: Option<&'static str>,
}

impl Default for XmlSerializer {
    fn default() -> Self {
        Self {
            output: String::new(),
            indent: "\t",
            depth: 0,
            start_root: Some(
                r###"<?xml version="1.0" encoding="ascii"?>
<hkpackfile classversion="8" contentsversion="hk_2010.2.0-r1" toplevelobject=""###,
            ),
            end_root: Some("</hkpackfile>\n"),
        }
    }
}

/// To XML String.
pub fn to_string<T>(value: &T, top_ptr: usize) -> Result<String>
where
    T: havok_serde::ser::Serialize,
{
    let mut serializer = XmlSerializer::default();

    if let Some(ref start_root) = serializer.start_root {
        serializer.output += start_root;
        serializer.output += &Pointer::new(top_ptr).to_string();
        serializer.output += "\">";
        serializer.output += "\n\n";
        serializer.increment_depth();
        serializer.indent();
        serializer.output += "<hksection name=\"__data__\">\n";
        serializer.increment_depth();
    };

    value.serialize(&mut serializer)?;

    if let Some(end_root) = serializer.end_root {
        serializer.decrement_depth();
        serializer.output += "\n";
        serializer.indent();
        serializer.output += "</hksection>";
        serializer.decrement_depth();
        serializer.output += "\n\n";
        serializer.output += end_root;
    };
    Ok(serializer.output)
}

impl<'a> havok_serde::ser::Serializer for &'a mut XmlSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeStruct = Self;
    type SerializeFlags = Self;

    #[inline]
    fn serialize_void(self, _: ()) -> Result<Self::Ok> {
        Ok(())
    }

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.output += if v { "true" } else { "false" };
        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_int8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_int64(v as i64)
    }

    #[inline]
    fn serialize_uint8(self, v: u8) -> Result<Self::Ok> {
        self.serialize_uint64(v as u64)
    }

    #[inline]
    fn serialize_int16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_int64(v as i64)
    }

    #[inline]
    fn serialize_uint16(self, v: u16) -> Result<Self::Ok> {
        self.serialize_uint64(v as u64)
    }

    #[inline]
    fn serialize_int32(self, v: i32) -> Result<Self::Ok> {
        self.serialize_int64(v as i64)
    }

    #[inline]
    fn serialize_uint32(self, v: u32) -> Result<Self::Ok> {
        self.serialize_uint64(v as u64)
    }

    #[inline]
    fn serialize_int64(self, v: i64) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_uint64(self, v: u64) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_real(self, v: f32) -> Result<Self::Ok> {
        self.output += &format!("{v:.06}");
        Ok(())
    }

    #[inline]
    fn serialize_vector4(self, v: &Vector4) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_quaternion(self, v: &Quaternion) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_matrix3(self, v: &Matrix3) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_rotation(self, v: &Rotation) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_qstransform(self, v: &QsTransform) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_matrix4(self, v: &Matrix4) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_transform(self, v: &Transform) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_pointer(self, v: Pointer) -> Result<Self::Ok> {
        if v.get() == 0 {
            self.output += "null"; // Null pointer
        } else {
            self.output += &v.to_string();
        }
        Ok(())
    }

    #[inline]
    fn serialize_array(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        class_meta: Option<(Pointer, Signature)>,
    ) -> Result<Self::SerializeStruct> {
        if let Some((ptr_name, sig)) = class_meta {
            self.output += "\n";
            self.indent();
            self.output +=
                &format!("<hkobject name=\"{ptr_name}\" class=\"{name}\" signature=\"{sig}\">\n");
        } else {
            self.indent();
            self.output += "<hkobject>\n";
        }

        // entered <hkparam>. so we increment indent.
        self.increment_depth();
        Ok(self)
    }

    /// TODO: Ignore the XML notation since it is unclear. (Fortunately this is only used for one class.)
    #[inline]
    fn serialize_variant(self, v: &Variant) -> Result<Self::Ok> {
        let _ = v;
        Ok(())
    }

    #[inline]
    fn serialize_cstring(self, v: &CString) -> Result<Self::Ok> {
        if let Some(s) = v.get_ref() {
            self.output += s;
        };
        Ok(())
    }

    #[inline]
    fn serialize_ulong(self, v: u64) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_enum_flags(self) -> Result<Self::SerializeFlags> {
        Ok(self)
    }

    #[inline]
    fn serialize_half(self, v: f16) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_stringptr(self, v: &StringPtr) -> Result<Self::Ok> {
        if let Some(s) = v.get_ref() {
            self.output += s;
        };
        Ok(())
    }
}

impl XmlSerializer {
    /// Do indentation by `self.depth`.
    fn indent(&mut self) {
        match self.depth {
            0 => (),
            1 => self.output += self.indent,
            _ => self.output += &self.indent.repeat(self.depth),
        }
    }

    /// Increment `self.depth` for indentation.
    #[inline]
    fn increment_depth(&mut self) {
        self.depth += 1;
    }

    /// Decrement `self.depth` for indentation.
    #[inline]
    fn decrement_depth(&mut self) {
        self.depth -= 1;
    }
}

impl<'a> havok_serde::ser::SerializeSeq for &'a mut XmlSerializer {
    type Ok = ();
    type Error = Error;

    /// # Expected XML Examples
    ///
    /// - `hkArray<hkInt8>`(same as ptr, hkReal, etc...)
    /// ```xml
    /// <hkparam name="key" numelements="20">
    ///     0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
    ///     16 17 18 19 20
    /// </hkparam>
    /// ```
    fn serialize_primitive_element<T>(
        &mut self,
        value: &T,
        index: usize,
        len: usize,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + havok_serde::ser::Serialize,
    {
        if index == 0 {
            self.indent();
        };

        value.serialize(&mut **self)?;
        self.output += " ";

        // Align the closing tag of `</hkparam>` by breaking the line at the end of the output,
        // regardless of whether it is every 16 or not.
        if index + 1 == len {
            self.output.push('\n');
            return Ok(());
        }

        // After 16 outputs, indent and make 16 columns.
        if (index + 1) % 16 == 0 {
            self.output.push('\n');
            self.indent();
        }

        Ok(())
    }

    fn serialize_class_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        self.output += "\n";
        Ok(())
    }

    fn serialize_math_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.indent();
        value.serialize(&mut **self)?;
        self.output += "\n";
        Ok(())
    }

    /// # XML Examples
    ///
    /// ```xml
    ///     <hkcstring>CString</hkcstring>
    /// ```
    fn serialize_cstring_element(&mut self, value: &CString) -> Result<()> {
        self.indent();
        self.output += "<hkcstring>";
        value.serialize(&mut **self)?;
        self.output += "</hkcstring>\n";
        Ok(())
    }

    /// # XML Examples
    ///
    /// ```xml
    ///     <hkcstring>StringPtr</hkcstring>
    /// ```
    fn serialize_stringptr_element(&mut self, value: &StringPtr) -> Result<()> {
        self.indent();
        self.output += "<hkcstring>";
        value.serialize(&mut **self)?;
        self.output += "</hkcstring>\n";
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> SerializeStruct for &'a mut XmlSerializer {
    type Ok = ();
    type Error = Error;

    /// # XML Examples
    ///
    /// ```xml
    /// <hkparam name="key1">value</hkparam>
    /// ```
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.indent();
        self.output += "<hkparam name=\"";
        self.output += key;
        self.output += "\">";

        value.serialize(&mut **self)?;

        self.output += "</hkparam>\n";
        Ok(())
    }

    #[inline]
    fn serialize_cstring_meta_field(&mut self, key: &'static str, value: &CString) -> Result<()> {
        SerializeStruct::serialize_field(self, key, value)
    }

    #[inline]
    fn serialize_stringptr_meta_field(
        &mut self,
        key: &'static str,
        value: &StringPtr,
    ) -> Result<()> {
        SerializeStruct::serialize_field(self, key, value)
    }

    /// # XML Examples
    ///
    /// - `hkArray<hkInt8>`(same as ptr, hkReal, etc...)
    /// ```xml
    /// <hkparam name="key" numelements="20">
    ///     0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
    ///     16 17 18 19 20
    /// </hkparam>
    /// ```
    ///
    /// - `hkArray<hkStringPtr>`
    /// ```xml
    /// <hkparam name="key" numelements="3">
    ///     <hkcstring>StringPtr1</hkcstring>
    ///     <hkcstring>StringPtr2</hkcstring>
    ///     <hkcstring>StringPtr3</hkcstring>
    /// </hkparam>
    /// ```
    ///
    /// - `hkArray<Vector4>`
    /// ```xml
    /// <hkparam name="key" numelements="2">
    ///     (0.000000 0.000000 0.000000 0.000000)
    ///     (0.000000 0.000000 0.000000 0.000000)
    /// </hkparam>
    /// ```
    fn serialize_array_meta_field<V, T>(&mut self, key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        self.indent();
        self.output += "<hkparam name=\"";
        self.output += key;

        let array = value.as_ref();
        if array.is_empty() {
            self.output += "\" numelements=\"0\"></hkparam>\n";
            return Ok(());
        };

        let len = array.len();
        self.output += &format!("\" numelements=\"{len}\">\n");
        self.increment_depth();
        value.serialize(&mut **self)?;
        self.decrement_depth();
        self.indent();
        self.output += "</hkparam>\n";
        Ok(())
    }

    /// # XML Examples
    ///
    /// ```xml
    /// <!-- key SERIALIZE_IGNORED --><!-- This is skip_field -->
    /// <hkparam name="otherKey"></hkparam>
    /// ```
    fn skip_field<T>(&mut self, key: &'static str, _: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.indent();
        self.output += &format!("<!-- {key} SERIALIZE_IGNORED -->\n");
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.decrement_depth();
        self.indent();
        self.output += "</hkobject>";
        Ok(())
    }
}

impl<'a> SerializeFlags for &'a mut XmlSerializer {
    type Ok = ();
    type Error = Error;

    /// e.g. <hkparam>0</hkparam>
    #[inline]
    fn serialize_empty_bit(&mut self) -> Result<(), Self::Error> {
        self.output += "0";
        Ok(())
    }

    fn serialize_field<T>(&mut self, key: &str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // Always prefix all flags except the first with `|` to indicate an OR operation.
        // e.g. <hkparam>EXAMPLE|EXAMPLE</hkparam>
        if !self.output.ends_with(">") {
            self.output += "|";
        }
        self.output += key;

        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::mocks::{classes::*, constructors::new_defaultmale};

    #[test]
    #[quick_tracing::try_init]
    fn test_serialize_types_all() -> Result<()> {
        let all_types_class = AllTypesTestClass {
            _name: Some(Pointer::new(11)),
            ..Default::default()
        };
        let mut classes = indexmap::IndexMap::new();
        classes.insert(11, Classes::AllTypesTestClass(all_types_class));

        tracing::trace!("\n{}", to_string(&classes, 11)?);
        Ok(())
    }

    #[test]
    fn test_serialize_defaultmale() -> Result<()> {
        let mut classes = new_defaultmale();

        // hkRootContainer" is processed last.
        classes.sort_keys();
        let mut top_ptr = None;
        if !classes.is_empty() {
            if let Some((first_key, first_value)) = classes.shift_remove_index(0) {
                classes.insert(first_key, first_value);
                top_ptr = Some(first_key);
            }
        }

        let actual = to_string(&classes, top_ptr.unwrap_or_default())?;
        let expected =
            include_str!("../../../docs/handson_hex_dump/defaultmale/defaultmale_x86.xml");
        pretty_assertions::assert_eq!(actual, expected);
        Ok(())
    }
}
