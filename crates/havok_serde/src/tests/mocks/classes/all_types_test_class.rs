use havok_types::{Pointer, Signature, StringPtr, Transform};

use crate::{ser::Serialize, tests::mocks::classes::HkReferencedObject};

use crate::tests::mocks::{enums::EventMode, flags::FlagValues};

#[derive(Debug, Default)]
pub struct AllTypesTestClass {
    pub name: Option<Pointer>,
}

impl crate::HavokClass for AllTypesTestClass {}

impl Serialize for AllTypesTestClass {
    fn serialize<S: crate::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use crate::ser::SerializeStruct;

        let class_meta = self.name.map(|name| (name, Signature::new(0x12345678)));
        let mut serializer = serializer.serialize_struct("AllTypesTestClass", class_meta)?;

        // Serialize fields of parent (flatten)
        serializer.skip_field("skip_test")?;

        serializer.serialize_field("bool", &true)?;
        serializer.serialize_field("char", &'C')?;
        serializer.serialize_field("u8", &8_u8)?;
        serializer.serialize_field("u16", &16_u16)?;
        serializer.serialize_field("u32", &32_u32)?;
        serializer.serialize_field("u64", &64_u64)?;
        serializer.serialize_field("i8", &-8_i8)?;
        serializer.serialize_field("i16", &-16_i16)?;
        serializer.serialize_field("i32", &-32_i32)?;
        serializer.serialize_field("i64", &-64_i64)?;
        serializer.serialize_field("f32", &1.0_f32)?;

        serializer.serialize_field("transform", &Transform::default())?;

        serializer.serialize_field("class_ptr", &Pointer::new(400))?;

        #[rustfmt::skip]
        serializer.serialize_array_field(
            "should_primitive_array_16_columns",
            &vec![
                 0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
                16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                32, 33, 34, 35, 36, 37,
            ],
        )?;
        #[rustfmt::skip]
        let floats = vec![
                 0.0,  1.0,  2.0,  3.0,  4.0,  5.0,  6.0,  7.0,  8.0,  9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0,
            ];
        serializer
            .serialize_array_field("should_primitive_array_16_columns_if_aligned16", &floats)?;

        serializer.serialize_array_field(
            "transform_array",
            vec![Transform::default(), Transform::default()],
        )?;
        serializer.serialize_array_field(
            "class_array",
            &vec![HkReferencedObject::default(), HkReferencedObject::default()],
        )?;

        serializer.serialize_field("string_ptr", &StringPtr::from("StringPtr"))?;

        serializer.serialize_field("enum", &EventMode::EventModeDefault)?;
        serializer.serialize_field("flags_none_is_0", &FlagValues::FLAGS_NONE)?;
        serializer.serialize_field(
            "no_display_0_flag_if_has_the_others",
            &(FlagValues::FLAGS_NONE | FlagValues::NOT_OWNED),
        )?;
        serializer.serialize_field(
            "flags_non_remain",
            &(FlagValues::NOT_OWNED | FlagValues::ALIGN_16),
        )?;
        serializer.serialize_field(
            "flags_with_remain",
            &(FlagValues::FLAGS_NONE
                | FlagValues::NOT_OWNED
                | FlagValues::SERIALIZE_IGNORED
                | FlagValues::from_bits_retain(1)),
        )?;

        serializer.end()
    }
}
