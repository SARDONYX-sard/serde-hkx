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
                r###"
<?xml version="1.0" encoding="ascii"?>
<hkpackfile classversion="8" contentsversion="hk_2010.2.0-r1" toplevelobject="#0050">

    <hksection name="__data__">"

"###
                .to_owned(),
            ),
            end_root: Some(
                r###"    </hksection>

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
    T: super::Serialize,
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

impl<'a> super::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeStruct = Self;

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
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_uint8(self, v: u8) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_int16(self, v: i16) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_uint16(self, v: u16) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_int32(self, v: i32) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_uint32(self, v: u32) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
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
        self.output += &v.to_string();
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
            self.output +=
                &format!("<hkobject name=\"{ptr_name}\" class=\"{name}\" signature=\"{sig}\">\n");
        } else {
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

    fn serialize_variant(self, v: u32) -> Result<Self::Ok> {
        let _ = v;
        todo!()
    }

    fn serialize_cstring(self, v: &CString) -> Result<Self::Ok> {
        self.output += &v;
        Ok(())
    }

    fn serialize_ulong(self, v: u64) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_flags(self, v: u32) -> Result<Self::Ok> {
        let _ = v;
        todo!()
    }

    fn serialize_half(self, v: f16) -> Result<Self::Ok> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_stringptr(self, s: &StringPtr) -> Result<Self::Ok> {
        self.output += &s;
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

impl<'a> super::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_class_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + super::Serialize,
    {
        self.indent();
        value.serialize(&mut **self)?;
        self.output += "\n";
        Ok(())
    }

    fn serialize_math_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + super::Serialize,
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
        T: ?Sized + super::Serialize,
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

impl<'a> super::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    /// # XML Examples
    ///
    /// ```xml
    /// <hkparam name="key1">value</hkparam>
    /// ```
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + super::Serialize,
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
        V: AsRef<[T]> + super::Serialize,
        T: super::Serialize,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ser::Serialize;
    use havok_types::{Pointer, Signature, Transform};

    #[derive(Debug, Clone, Default, PartialEq)]
    struct HkReferencedObject {
        pub name: Option<Pointer>,

        /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
        /// -   name:`"memSizeAndFlags"`
        /// -   type: `hkUint16`
        /// - offset: 4
        /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
        pub mem_size_and_flags: u16,
        /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
        /// -   name:`"referenceCount"`
        /// -   type: `hkInt16`
        /// - offset: 6
        /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
        pub reference_count: i16,
    }

    impl crate::HavokClass for HkReferencedObject {}
    impl Serialize for HkReferencedObject {
        fn serialize<S: crate::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use crate::ser::SerializeStruct;

            let class_meta = self.name.map(|name| (name, Signature::new(0xea7f1d08)));
            let mut serializer = serializer.serialize_struct("hkReferencedObject", class_meta)?;

            serializer.skip_field("referenceCount")?;
            serializer.skip_field("memSizeAndFlags")?;
            serializer.end()
        }
    }

    /// # C++ Class Info
    /// -      size: 112
    /// -    vtable: true
    /// -    parent: `hkReferencedObject`/`0x3b1c1113`
    /// - signature: `0xea7f1d08`
    /// -   version: 0
    #[derive(Debug, Clone, Default, PartialEq)]
    struct HkpShapeInfo<'a> {
        pub parent: HkReferencedObject,

        pub name: Option<Pointer>,

        /// # C++ Class Fields Info
        /// -   name:`"shape"`
        /// -   type: `struct hkpShape*`
        /// - offset: 8
        /// -  flags: `FLAGS_NONE`
        pub shape: Pointer,
        /// # C++ Class Fields Info
        /// -   name:`"isHierarchicalCompound"`
        /// -   type: `hkBool`
        /// - offset: 12
        /// -  flags: `FLAGS_NONE`
        pub is_hierarchical_compound: bool,
        /// # C++ Class Fields Info
        /// -   name:`"hkdShapesCollected"`
        /// -   type: `hkBool`
        /// - offset: 13
        /// -  flags: `FLAGS_NONE`
        pub hkd_shapes_collected: bool,
        /// # C++ Class Fields Info
        /// -   name:`"childShapeNames"`
        /// -   type: `hkArray<hkStringPtr>`
        /// - offset: 16
        /// -  flags: `FLAGS_NONE`
        pub child_shape_names: Vec<StringPtr<'a>>,
        /// # C++ Class Fields Info
        /// -   name:`"childTransforms"`
        /// -   type: `hkArray<hkTransform>`
        /// - offset: 28
        /// -  flags: `FLAGS_NONE`
        pub child_transforms: Vec<Transform>,
        /// # C++ Class Fields Info
        /// -   name:`"transform"`
        /// -   type: `hkTransform`
        /// - offset: 48
        /// -  flags: `FLAGS_NONE`
        pub transform: Transform,

        pub v: Vec<HkReferencedObject>,
    }

    impl crate::HavokClass for HkpShapeInfo<'_> {}
    impl Serialize for HkpShapeInfo<'_> {
        fn serialize<S: crate::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use crate::ser::SerializeStruct;

            let class_meta = self.name.map(|name| (name, Signature::new(0xea7f1d08)));
            let mut serializer = serializer.serialize_struct("hkpShapeInfo", class_meta)?;

            // Serialize fields of parent (flatten)
            serializer.skip_field("memSizeAndFlags")?;
            serializer.skip_field("referenceCount")?;

            serializer.serialize_field("shape", &self.shape)?;
            serializer.serialize_field("isHierarchicalCompound", &self.is_hierarchical_compound)?;
            serializer.serialize_field("hkdShapesCollected", &self.hkd_shapes_collected)?;
            serializer.serialize_array_field("childShapeNames", &self.child_shape_names)?;
            serializer.serialize_array_field("childTransforms", &self.child_transforms)?;
            serializer.serialize_field("transform", &self.transform)?;
            serializer.serialize_array_field("v", &self.v)?;
            serializer.end()
        }
    }

    enum Classes<'a> {
        HkpShapeInfo(HkpShapeInfo<'a>),
        HkReferencedObject(HkReferencedObject),
    }

    impl crate::HavokClass for Classes<'_> {}
    impl<'a> Serialize for Classes<'a> {
        fn serialize<S: crate::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Classes::HkpShapeInfo(class) => class.serialize(serializer),
                Classes::HkReferencedObject(class) => class.serialize(serializer),
            }
        }
    }

    #[test]
    fn test_serialize() {
        let hk_referenced_object = HkReferencedObject {
            name: Some(51.into()),
            mem_size_and_flags: 5,
            reference_count: 6,
        };

        let hk_referenced_object_inline = HkReferencedObject {
            name: None,
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
            v: vec![
                hk_referenced_object_inline.clone(),
                hk_referenced_object_inline,
            ],
            ..Default::default()
        };

        let classes = vec![
            Classes::HkpShapeInfo(hkp_shape_info.clone()),
            Classes::HkpShapeInfo(hkp_shape_info),
            Classes::HkReferencedObject(hk_referenced_object),
        ];
        println!("{}", to_string(&classes).unwrap());
    }
}
