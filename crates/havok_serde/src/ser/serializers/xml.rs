//! XML Serialization
use crate::lib::*;

use crate::error::{Error, Result};
use havok_types::{
    f16, CString, Matrix3, Matrix4, Pointer, QsTransform, Quaternion, Rotation, Signature,
    StringPtr, Transform, Vector4,
};

#[derive(Debug)]
struct Serializer {
    output: String,
    indent: &'static str,
    depth: usize,
    start_root: Option<String>,
    end_root: Option<String>,
}

impl Default for Serializer {
    fn default() -> Self {
        Self {
            output: String::new(),
            indent: "    ",
            depth: 2,
            start_root: Some(
                r###"<?xml version="1.0" encoding="ascii"?>
<hkpackfile classversion="8" contentsversion="hk_2010.2.0-r1" toplevelobject="#0050">

    <hksection name="__data__">"
"###
                .to_owned(),
            ),
            end_root: Some(
                r###"
    </hksection>

</hkpackfile>
"###
                .to_owned(),
            ),
        }
    }
}

/// To XML String.
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: crate::ser::Serialize,
{
    let mut serializer = Serializer::default();

    if let Some(ref start_root) = serializer.start_root {
        serializer.output += start_root;
    };

    value.serialize(&mut serializer)?;

    if let Some(ref end_root) = serializer.end_root {
        serializer.output += end_root;
    };
    Ok(serializer.output)
}

impl<'a> crate::ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeStruct = Self;
    type SerializeFlags = Self;

    fn serialize_void(self, _: ()) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.output += if v { "true" } else { "false" };
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_int8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_int64(v as i64)
    }

    fn serialize_uint8(self, v: u8) -> Result<Self::Ok> {
        self.serialize_uint64(v as u64)
    }

    fn serialize_int16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_int64(v as i64)
    }

    fn serialize_uint16(self, v: u16) -> Result<Self::Ok> {
        self.serialize_uint64(v as u64)
    }

    fn serialize_int32(self, v: i32) -> Result<Self::Ok> {
        self.serialize_int64(v as i64)
    }

    fn serialize_uint32(self, v: u32) -> Result<Self::Ok> {
        self.serialize_uint64(v as u64)
    }

    fn serialize_int64(self, v: i64) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_uint64(self, v: u64) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_real(self, v: f32) -> Result<Self::Ok> {
        self.output += &format!("{v:.06}");
        Ok(())
    }

    fn serialize_vector4(self, v: &Vector4) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_quaternion(self, v: &Quaternion) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_matrix3(self, v: &Matrix3) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_rotation(self, v: &Rotation) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_qstransform(self, v: &QsTransform) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_matrix4(self, v: &Matrix4) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_transform(self, v: &Transform) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_zero(self, _: ()) -> Result<Self::Ok> {
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

    fn serialize_functionpointer(self, _: ()) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_array(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_inplacearray(self, _: ()) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_enum(self, v: u32) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
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

    fn serialize_simplearray(self, _: ()) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_homogeneousarray(self, _: ()) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_variant(self, v: u64) -> Result<Self::Ok> {
        let _ = v;
        Ok(())
    }

    fn serialize_cstring(self, v: &CString) -> Result<Self::Ok> {
        self.output += &v;
        Ok(())
    }

    fn serialize_ulong(self, v: u64) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_flags(self) -> Result<Self::SerializeFlags> {
        Ok(self)
    }

    fn serialize_half(self, v: f16) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_stringptr(self, s: &StringPtr) -> Result<Self::Ok> {
        self.output += s;
        Ok(())
    }

    fn serialize_relarray(self, _: ()) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_max(self, _: ()) -> Result<Self::Ok> {
        Ok(())
    }
}

impl Serializer {
    /// Do indentation by `self.depth`.
    fn indent(&mut self) {
        self.output += &self.indent.repeat(self.depth);
    }

    /// Increment `self.depth` for indentation.
    fn increment_depth(&mut self) {
        self.depth += 1;
    }

    /// Decrement `self.depth` for indentation.
    fn decrement_depth(&mut self) {
        self.depth -= 1;
    }
}

impl<'a> crate::ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_primitive_element<T>(
        &mut self,
        value: &T,
        index: usize,
        len: usize,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + crate::ser::Serialize,
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
        T: ?Sized + crate::ser::Serialize,
    {
        value.serialize(&mut **self)?;
        self.output += "\n";
        Ok(())
    }

    fn serialize_math_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + crate::ser::Serialize,
    {
        self.indent();
        value.serialize(&mut **self)?;
        self.output += "\n";
        Ok(())
    }

    /// # XML Examples
    ///
    /// ```xml
    ///     <hkcstring>StringPtr</hkcstring>
    /// ```
    fn serialize_string_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + crate::ser::Serialize,
    {
        self.indent();
        self.output += "<hkcstring>";
        value.serialize(&mut **self)?;
        self.output += "</hkcstring>\n";
        Ok(())
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> crate::ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    /// # XML Examples
    ///
    /// ```xml
    /// <hkparam name="key1">value</hkparam>
    /// ```
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + crate::ser::Serialize,
    {
        self.indent();
        self.output += "<hkparam name=\"";
        self.output += key;
        self.output += "\">";

        value.serialize(&mut **self)?;

        self.output += "</hkparam>\n";
        Ok(())
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
    fn serialize_array_field<V, T>(&mut self, key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + crate::ser::Serialize,
        T: crate::ser::Serialize,
    {
        let len = value.as_ref().len();
        self.indent();
        self.output += &format!("<hkparam name=\"{key}\" numelements=\"{len}\">\n");

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
    fn skip_field(&mut self, key: &'static str) -> Result<()> {
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

impl<'a> crate::ser::SerializeFlags for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_empty_bit(&mut self) -> Result<(), Self::Error> {
        self.output += "0";
        Ok(())
    }

    fn serialize_field<T>(&mut self, key: &str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + crate::ser::Serialize,
    {
        if !self.output.ends_with(">") {
            self.output += "|";
        }
        self.output += key;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::mocks::classes::{
        AllTypesTestClass, Classes, HkReferencedObject, HkpShapeInfo,
    };

    #[test]
    fn test_serialize() {
        let hk_referenced_object = HkReferencedObject {
            name: Some(51.into()),
            mem_size_and_flags: 5,
            reference_count: 6,
        };

        let hkp_shape_info = HkpShapeInfo {
            name: Some(50.into()),
            shape: Pointer::new(50),
            is_hierarchical_compound: true,
            hkd_shapes_collected: false,
            child_shape_names: vec!["child".into(), "Hi".into()],
            child_transforms: vec![
                Transform::default(),
                Transform::default(),
                Transform::default(),
            ],
            ..Default::default()
        };

        let classes = vec![
            Classes::HkpShapeInfo(hkp_shape_info.clone()),
            Classes::HkpShapeInfo(hkp_shape_info),
            Classes::HkReferencedObject(hk_referenced_object),
            Classes::AllTypesTestClass(AllTypesTestClass {
                name: Some(150.into()),
                ..Default::default()
            }),
        ];

        println!("{}", to_string(&classes).unwrap());
    }
}
