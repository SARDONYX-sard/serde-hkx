//! XML Serialization
use crate::{lib::*, tri};

use crate::errors::ser::{Error, Result};
use havok_serde::ser::{Serialize, SerializeFlags, SerializeStruct, Serializer};
use havok_types::variant::Variant;
use havok_types::{
    f16, CString, Matrix3, Matrix4, Pointer, QsTransform, Quaternion, Rotation, Signature,
    StringPtr, Transform, Ulong, Vector4,
};

#[derive(Debug)]
pub struct XmlSerializer {
    /// XML string
    output: String,
    /// Indent type(tab, space)
    indent: &'static str,
    /// Indent time
    depth: usize,
    /// If you want to output XML partially, put [`Option::None`].
    /// # Example XML
    /// ```xml
    /// <?xml version="1.0" encoding="ascii"?>
    /// <hkpackfile classversion="8" contentsversion="hk_2010.2.0-r1" toplevelobject=""
    /// ```
    start_root: Option<&'static str>,
    /// If you want to output XML partially, put [`Option::None`].
    /// # Example XML
    /// ```xml
    /// </hkpackfile>
    /// ```
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
#[inline]
pub fn to_string<T>(value: &T, top_ptr: usize) -> Result<String>
where
    T: Serialize,
{
    to_string_with_opt(value, top_ptr, XmlSerializer::default())
}

/// To xml string with custom `XmlSerializer` settings.
///
/// # Info
/// This can be done in partial mode by eliminating the root string.
pub fn to_string_with_opt<T>(value: &T, top_ptr: usize, ser: XmlSerializer) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = ser;

    if let Some(start_root) = serializer.start_root {
        serializer.output += start_root;
        serializer.output += &Pointer::new(top_ptr).to_string();
        serializer.output += "\">\n\n";
        serializer.increment_depth();
        serializer.indent();
        serializer.output += "<hksection name=\"__data__\">\n";
        serializer.increment_depth();
    };

    tri!(value.serialize(&mut serializer));

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

impl<'a> Serializer for &'a mut XmlSerializer {
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

    /// Create an XML string like the following
    /// ```xml
    /// <hkobject name="#0010" class="hkbProjectData" signature="0x13a39ba7">
    ///   <!-- memSizeAndFlags SERIALIZE_IGNORED -->
    ///   <!-- referenceCount SERIALIZE_IGNORED -->
    ///   <hkparam name="worldUpWS">(0.000000 0.000000 1.000000 0.000000)</hkparam>
    ///   <hkparam name="stringData">#0009</hkparam>
    ///   <hkparam name="defaultEventMode">EVENT_MODE_IGNORE_FROM_GENERATOR</hkparam>
    /// </hkobject>
    /// ```
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
            // If there is a line break in the class of a single field, but it is impossible to
            // determine whether it is a class or not within the serialize method of a single
            // field, so if there is no line break here, it is processed as class processing
            // within the field.
            if !self.output.ends_with('\n') {
                self.output += "\n";
                self.increment_depth();
            };
            self.indent();
            self.output += "<hkobject>\n"; // If ptr & signature are not provided, the class is considered to be an in-field class. (e.g. `Array<hkRootContainerNamedVariant>`)
        }

        self.increment_depth(); // entered <hkparam>(each fields process). so we increment indent.
        Ok(self)
    }

    /// FIXME: Unclear XML representation
    #[inline]
    fn serialize_variant(self, v: &Variant) -> Result<Self::Ok> {
        tri!(self.serialize_pointer(v.object));
        self.serialize_pointer(v.class)
    }

    fn serialize_cstring(self, v: &CString) -> Result<Self::Ok> {
        if let Some(s) = v.get_ref() {
            self.output += &html_escape::encode_text(s);
        } else {
            // null is &#9216: https://www.compart.com/en/unicode/U+2400
            self.output += "&#9216;";
        };
        Ok(())
    }

    #[inline]
    fn serialize_ulong(self, v: Ulong) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    #[inline]
    fn serialize_enum_flags(self) -> Result<Self::SerializeFlags> {
        Ok(self)
    }

    #[inline]
    fn serialize_half(self, v: f16) -> Result<Self::Ok> {
        self.output += &format!("{v:.06}");
        Ok(())
    }

    fn serialize_stringptr(self, v: &StringPtr) -> Result<Self::Ok> {
        if let Some(s) = v.get_ref() {
            self.output += &html_escape::encode_text(s);
        } else {
            // null is &#9216: https://www.compart.com/en/unicode/U+2400
            self.output += "&#9216;";
        };
        Ok(())
    }
}

impl XmlSerializer {
    /// Do indentation by `self.depth`.
    #[inline]
    fn indent(&mut self) {
        match self.depth {
            // Heap alloc optimizations
            0 => (),                                             // 0 alloc
            1 => self.output += self.indent,                     // 1 time alloc
            _ => self.output += &self.indent.repeat(self.depth), // 2 time alloc
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
        tri!(value.serialize(&mut **self));

        if index + 1 == len {
            // Align the closing tag of `</hkparam>` by breaking the line at the end of the output,
            // regardless of whether it is every 16 or not.
            self.output.push('\n');
            return Ok(());
        } else if (index + 1) % 16 == 0 {
            self.output.push('\n'); // After 16 outputs, indent and make 16 columns.
            self.indent();
        } else {
            self.output.push(' '); // add space each element.
        }
        Ok(())
    }

    fn serialize_class_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        tri!(value.serialize(&mut **self));
        self.output += "\n";
        Ok(())
    }

    fn serialize_math_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.indent();
        tri!(value.serialize(&mut **self));
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
        tri!(value.serialize(&mut **self));
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
        tri!(value.serialize(&mut **self));
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

        tri!(value.serialize(&mut **self));

        // The class in the field is currently having line break processing problems,
        // so it is processed to format it well. from `serialize_struct`
        if self.output.ends_with("</hkobject>") {
            self.output += "\n";
            self.decrement_depth();
            self.indent();
        };
        self.output += "</hkparam>\n";
        Ok(())
    }

    #[inline]
    fn serialize_fixed_array_field<V, T>(
        &mut self,
        key: &'static str,
        value: V,
    ) -> std::result::Result<(), Self::Error>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        SerializeStruct::serialize_array_meta_field(self, key, value)
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
        tri!(value.serialize(&mut **self));
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
        if !self.output.ends_with('>') {
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

    #[ignore = "No error on local PC Windows, but for some reason error occurs on GitHub Actions Windows"]
    #[test]
    fn test_serialize_defaultmale() -> Result<()> {
        use crate::tests::mocks::new_defaultmale;

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

        let actual = tri!(to_string(&classes, top_ptr.unwrap_or_default()));
        let expected =
            include_str!("../../../../docs/handson_hex_dump/defaultmale/defaultmale_x86.xml");

        pretty_assertions::assert_eq!(actual, expected);
        Ok(())
    }
}
