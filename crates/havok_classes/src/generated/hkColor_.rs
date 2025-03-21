use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkColor`
/// - version: `0`
/// - signature: `0x106b96ce`
/// - size: `  1`(x86)/`  1`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkColor<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub __ptr: Option<Pointer<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkColor<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkColor"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x106b96ce)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkColor<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x106b96ce)));
            let mut serializer = __serializer
                .serialize_struct("hkColor", class_meta, (1u64, 1u64))?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkColor<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hkColorVisitor<'de> {
                marker: _serde::__private::PhantomData<hkColor<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkColorVisitor<'de> {
                type Value = hkColor<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkColor")
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    for i in 0..0usize {
                        match i {
                            _ => {}
                        }
                    }
                    __A::pad(&mut __map, 1usize, 1usize)?;
                    _serde::__private::Ok(hkColor { __ptr })
                }
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkColor { __ptr: __ptr.clone() })
                }
            }
            const FIELDS: &[&str] = &[];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkColor",
                FIELDS,
                __hkColorVisitor {
                    marker: _serde::__private::PhantomData::<hkColor>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Enum as bit flags"] #[doc = r""] #[doc = r" # C++ Info"] #[doc =
    " - name: `ExtendedColors`(ctype: `hkEnum<ExtendedColors, hkUint32>`)"] #[doc = r""]
    #[doc = r" # Why this `enum` defined as `bitflags`?"] #[doc =
    r" Since Rust does not allow the definition of `enum` with duplicate discriminant values, they are defined as `bitflags`."]
    #[allow(non_upper_case_globals, non_snake_case)] #[cfg_attr(feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    ExtendedColors : u32 { #[doc = "-8388608"] const MAROON = 4286578688u32; #[doc =
    "-7667712"] const DARKRED = 4287299584u32; #[doc = "-65536"] const RED =
    4294901760u32; #[doc = "-18751"] const LIGHTPINK = 4294948545u32; #[doc = "-2354116"]
    const CRIMSON = 4292613180u32; #[doc = "-2396013"] const PALEVIOLETRED =
    4292571283u32; #[doc = "-38476"] const HOTPINK = 4294928820u32; #[doc = "-60269"]
    const DEEPPINK = 4294907027u32; #[doc = "-3730043"] const MEDIUMVIOLETRED =
    4291237253u32; #[doc = "-8388480"] const PURPLE = 4286578816u32; #[doc = "-7667573"]
    const DARKMAGENTA = 4287299723u32; #[doc = "-2461482"] const ORCHID = 4292505814u32;
    #[doc = "-2572328"] const THISTLE = 4292394968u32; #[doc = "-2252579"] const PLUM =
    4292714717u32; #[doc = "-1146130"] const VIOLET = 4293821166u32; #[doc = "-65281"]
    const FUCHSIA = 4294902015u32; #[doc = "-65281"] const MAGENTA = 4294902015u32; #[doc
    = "-4565549"] const MEDIUMORCHID = 4290401747u32; #[doc = "-7077677"] const
    DARKVIOLET = 4287889619u32; #[doc = "-6737204"] const DARKORCHID = 4288230092u32;
    #[doc = "-7722014"] const BLUEVIOLET = 4287245282u32; #[doc = "-11861886"] const
    INDIGO = 4283105410u32; #[doc = "-7114533"] const MEDIUMPURPLE = 4287852763u32; #[doc
    = "-9807155"] const SLATEBLUE = 4285160141u32; #[doc = "-8689426"] const
    MEDIUMSLATEBLUE = 4286277870u32; #[doc = "-16777077"] const DARKBLUE = 4278190219u32;
    #[doc = "-16777011"] const MEDIUMBLUE = 4278190285u32; #[doc = "-16776961"] const
    BLUE = 4278190335u32; #[doc = "-16777088"] const NAVY = 4278190208u32; #[doc =
    "-15132304"] const MIDNIGHTBLUE = 4279834992u32; #[doc = "-12042869"] const
    DARKSLATEBLUE = 4282924427u32; #[doc = "-12490271"] const ROYALBLUE = 4282477025u32;
    #[doc = "-10185235"] const CORNFLOWERBLUE = 4284782061u32; #[doc = "-5192482"] const
    LIGHTSTEELBLUE = 4289774814u32; #[doc = "-984833"] const ALICEBLUE = 4293982463u32;
    #[doc = "-460545"] const GHOSTWHITE = 4294506751u32; #[doc = "-1644806"] const
    LAVENDER = 4293322490u32; #[doc = "-14774017"] const DODGERBLUE = 4280193279u32;
    #[doc = "-12156236"] const STEELBLUE = 4282811060u32; #[doc = "-16728065"] const
    DEEPSKYBLUE = 4278239231u32; #[doc = "-9404272"] const SLATEGRAY = 4285563024u32;
    #[doc = "-8943463"] const LIGHTSLATEGRAY = 4286023833u32; #[doc = "-7876870"] const
    LIGHTSKYBLUE = 4287090426u32; #[doc = "-7876885"] const SKYBLUE = 4287090411u32;
    #[doc = "-5383962"] const LIGHTBLUE = 4289583334u32; #[doc = "-16744320"] const TEAL
    = 4278222976u32; #[doc = "-16741493"] const DARKCYAN = 4278225803u32; #[doc =
    "-16724271"] const DARKTURQUOISE = 4278243025u32; #[doc = "-16711681"] const CYAN =
    4278255615u32; #[doc = "-12004916"] const MEDIUMTURQUOISE = 4282962380u32; #[doc =
    "-10510688"] const CADETBLUE = 4284456608u32; #[doc = "-5247250"] const PALETURQUOISE
    = 4289720046u32; #[doc = "-2031617"] const LIGHTCYAN = 4292935679u32; #[doc =
    "-983041"] const AZURE = 4293984255u32; #[doc = "-14634326"] const LIGHTSEAGREEN =
    4280332970u32; #[doc = "-12525360"] const TURQUOISE = 4282441936u32; #[doc =
    "-5185306"] const POWDERBLUE = 4289781990u32; #[doc = "-13676721"] const
    DARKSLATEGRAY = 4281290575u32; #[doc = "-8388652"] const AQUAMARINE = 4286578644u32;
    #[doc = "-16713062"] const MEDIUMSPRINGGREEN = 4278254234u32; #[doc = "-10039894"]
    const MEDIUMAQUAMARINE = 4284927402u32; #[doc = "-16711809"] const SPRINGGREEN =
    4278255487u32; #[doc = "-12799119"] const MEDIUMSEAGREEN = 4282168177u32; #[doc =
    "-13726889"] const SEAGREEN = 4281240407u32; #[doc = "-13447886"] const LIMEGREEN =
    4281519410u32; #[doc = "-16751616"] const DARKGREEN = 4278215680u32; #[doc =
    "-16744448"] const GREEN = 4278222848u32; #[doc = "-16711936"] const LIME =
    4278255360u32; #[doc = "-14513374"] const FORESTGREEN = 4280453922u32; #[doc =
    "-7357297"] const DARKSEAGREEN = 4287609999u32; #[doc = "-7278960"] const LIGHTGREEN
    = 4287688336u32; #[doc = "-6751336"] const PALEGREEN = 4288215960u32; #[doc =
    "-655366"] const MINTCREAM = 4294311930u32; #[doc = "-983056"] const HONEYDEW =
    4293984240u32; #[doc = "-8388864"] const CHARTREUSE = 4286578432u32; #[doc =
    "-8586240"] const LAWNGREEN = 4286381056u32; #[doc = "-9728477"] const OLIVEDRAB =
    4285238819u32; #[doc = "-11179217"] const DARKOLIVEGREEN = 4283788079u32; #[doc =
    "-6632142"] const YELLOWGREEN = 4288335154u32; #[doc = "-5374161"] const GREENYELLOW
    = 4289593135u32; #[doc = "-657956"] const BEIGE = 4294309340u32; #[doc = "-331546"]
    const LINEN = 4294635750u32; #[doc = "-329006"] const LIGHTGOLDENRODYELLOW =
    4294638290u32; #[doc = "-8355840"] const OLIVE = 4286611456u32; #[doc = "-256"] const
    YELLOW = 4294967040u32; #[doc = "-32"] const LIGHTYELLOW = 4294967264u32; #[doc =
    "-16"] const IVORY = 4294967280u32; #[doc = "-4343957"] const DARKKHAKI =
    4290623339u32; #[doc = "-989556"] const KHAKI = 4293977740u32; #[doc = "-1120086"]
    const PALEGOLDENROD = 4293847210u32; #[doc = "-663885"] const WHEAT = 4294303411u32;
    #[doc = "-10496"] const GOLD = 4294956800u32; #[doc = "-1331"] const LEMONCHIFFON =
    4294965965u32; #[doc = "-4139"] const PAPAYAWHIP = 4294963157u32; #[doc = "-4684277"]
    const DARKGOLDENROD = 4290283019u32; #[doc = "-2448096"] const GOLDENROD =
    4292519200u32; #[doc = "-332841"] const ANTIQUEWHITE = 4294634455u32; #[doc =
    "-1828"] const CORNSILK = 4294965468u32; #[doc = "-133658"] const OLDLACE =
    4294833638u32; #[doc = "-6987"] const MOCCASIN = 4294960309u32; #[doc = "-8531"]
    const NAVAJOWHITE = 4294958765u32; #[doc = "-23296"] const ORANGE = 4294944000u32;
    #[doc = "-6972"] const BISQUE = 4294960324u32; #[doc = "-2968436"] const TAN =
    4291998860u32; #[doc = "-29696"] const DARKORANGE = 4294937600u32; #[doc =
    "-2180985"] const BURLYWOOD = 4292786311u32; #[doc = "-7650029"] const SADDLEBROWN =
    4287317267u32; #[doc = "-744352"] const SANDYBROWN = 4294222944u32; #[doc = "-5171"]
    const BLANCHEDALMOND = 4294962125u32; #[doc = "-3851"] const LAVENDERBLUSH =
    4294963445u32; #[doc = "-2578"] const SEASHELL = 4294964718u32; #[doc = "-1296"]
    const FLORALWHITE = 4294966000u32; #[doc = "-1286"] const SNOW = 4294966010u32; #[doc
    = "-3308225"] const PERU = 4291659071u32; #[doc = "-9543"] const PEACHPUFF =
    4294957753u32; #[doc = "-2987746"] const CHOCOLATE = 4291979550u32; #[doc =
    "-6270419"] const SIENNA = 4288696877u32; #[doc = "-24454"] const LIGHTSALMON =
    4294942842u32; #[doc = "-32944"] const CORAL = 4294934352u32; #[doc = "-1468806"]
    const DARKSALMON = 4293498490u32; #[doc = "-6943"] const MISTYROSE = 4294960353u32;
    #[doc = "-47872"] const ORANGERED = 4294919424u32; #[doc = "-360334"] const SALMON =
    4294606962u32; #[doc = "-40121"] const TOMATO = 4294927175u32; #[doc = "-4419697"]
    const ROSYBROWN = 4290547599u32; #[doc = "-16181"] const PINK = 4294951115u32; #[doc
    = "-3318692"] const INDIANRED = 4291648604u32; #[doc = "-1015680"] const LIGHTCORAL =
    4293951616u32; #[doc = "-5952982"] const BROWN = 4289014314u32; #[doc = "-5103070"]
    const FIREBRICK = 4289864226u32; #[doc = "-16777216"] const BLACK = 4278190080u32;
    #[doc = "-9868951"] const DIMGRAY = 4285098345u32; #[doc = "-8355712"] const GRAY =
    4286611584u32; #[doc = "-5658199"] const DARKGRAY = 4289309097u32; #[doc =
    "-4144960"] const SILVER = 4290822336u32; #[doc = "-2894893"] const LIGHTGREY =
    4292072403u32; #[doc = "-2302756"] const GAINSBORO = 4292664540u32; #[doc =
    "-657931"] const WHITESMOKE = 4294309365u32; #[doc = "-1"] const WHITE =
    4294967295u32; #[doc = "-7829368"] const GREY = 4287137928u32; #[doc = "-12566464"]
    const GREY25 = 4282400832u32; #[doc = "-8355712"] const GREY50 = 4286611584u32; #[doc
    = "-4144960"] const GREY75 = 4290822336u32; }
}
#[cfg(feature = "json_schema")]
const _: () = {
    use schemars::{SchemaGenerator, Schema, JsonSchema, json_schema};
    use std::borrow::Cow;
    impl JsonSchema for ExtendedColors {
        fn schema_name() -> Cow<'static, str> {
            "ExtendedColors".into()
        }
        fn schema_id() -> Cow<'static, str> {
            concat!(module_path!(), "::", "ExtendedColors").into()
        }
        fn json_schema(generator: &mut SchemaGenerator) -> Schema {
            use schemars::_private::serde_json::{self, Value};
            let selection = &[
                "MAROON",
                "DARKRED",
                "RED",
                "LIGHTPINK",
                "CRIMSON",
                "PALEVIOLETRED",
                "HOTPINK",
                "DEEPPINK",
                "MEDIUMVIOLETRED",
                "PURPLE",
                "DARKMAGENTA",
                "ORCHID",
                "THISTLE",
                "PLUM",
                "VIOLET",
                "FUCHSIA",
                "MAGENTA",
                "MEDIUMORCHID",
                "DARKVIOLET",
                "DARKORCHID",
                "BLUEVIOLET",
                "INDIGO",
                "MEDIUMPURPLE",
                "SLATEBLUE",
                "MEDIUMSLATEBLUE",
                "DARKBLUE",
                "MEDIUMBLUE",
                "BLUE",
                "NAVY",
                "MIDNIGHTBLUE",
                "DARKSLATEBLUE",
                "ROYALBLUE",
                "CORNFLOWERBLUE",
                "LIGHTSTEELBLUE",
                "ALICEBLUE",
                "GHOSTWHITE",
                "LAVENDER",
                "DODGERBLUE",
                "STEELBLUE",
                "DEEPSKYBLUE",
                "SLATEGRAY",
                "LIGHTSLATEGRAY",
                "LIGHTSKYBLUE",
                "SKYBLUE",
                "LIGHTBLUE",
                "TEAL",
                "DARKCYAN",
                "DARKTURQUOISE",
                "CYAN",
                "MEDIUMTURQUOISE",
                "CADETBLUE",
                "PALETURQUOISE",
                "LIGHTCYAN",
                "AZURE",
                "LIGHTSEAGREEN",
                "TURQUOISE",
                "POWDERBLUE",
                "DARKSLATEGRAY",
                "AQUAMARINE",
                "MEDIUMSPRINGGREEN",
                "MEDIUMAQUAMARINE",
                "SPRINGGREEN",
                "MEDIUMSEAGREEN",
                "SEAGREEN",
                "LIMEGREEN",
                "DARKGREEN",
                "GREEN",
                "LIME",
                "FORESTGREEN",
                "DARKSEAGREEN",
                "LIGHTGREEN",
                "PALEGREEN",
                "MINTCREAM",
                "HONEYDEW",
                "CHARTREUSE",
                "LAWNGREEN",
                "OLIVEDRAB",
                "DARKOLIVEGREEN",
                "YELLOWGREEN",
                "GREENYELLOW",
                "BEIGE",
                "LINEN",
                "LIGHTGOLDENRODYELLOW",
                "OLIVE",
                "YELLOW",
                "LIGHTYELLOW",
                "IVORY",
                "DARKKHAKI",
                "KHAKI",
                "PALEGOLDENROD",
                "WHEAT",
                "GOLD",
                "LEMONCHIFFON",
                "PAPAYAWHIP",
                "DARKGOLDENROD",
                "GOLDENROD",
                "ANTIQUEWHITE",
                "CORNSILK",
                "OLDLACE",
                "MOCCASIN",
                "NAVAJOWHITE",
                "ORANGE",
                "BISQUE",
                "TAN",
                "DARKORANGE",
                "BURLYWOOD",
                "SADDLEBROWN",
                "SANDYBROWN",
                "BLANCHEDALMOND",
                "LAVENDERBLUSH",
                "SEASHELL",
                "FLORALWHITE",
                "SNOW",
                "PERU",
                "PEACHPUFF",
                "CHOCOLATE",
                "SIENNA",
                "LIGHTSALMON",
                "CORAL",
                "DARKSALMON",
                "MISTYROSE",
                "ORANGERED",
                "SALMON",
                "TOMATO",
                "ROSYBROWN",
                "PINK",
                "INDIANRED",
                "LIGHTCORAL",
                "BROWN",
                "FIREBRICK",
                "BLACK",
                "DIMGRAY",
                "GRAY",
                "DARKGRAY",
                "SILVER",
                "LIGHTGREY",
                "GAINSBORO",
                "WHITESMOKE",
                "WHITE",
                "GREY",
                "GREY25",
                "GREY50",
                "GREY75",
            ];
            let selection = selection
                .iter()
                .map(|s| Value::String(s.to_string()))
                .collect();
            let mut schema = Value::json_schema(generator);
            let mut map = schema.ensure_object();
            map.insert("type".to_string(), Value::String("string".to_string()));
            map.insert("enum".to_string(), Value::Array(selection));
            schema
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ExtendedColors {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match *self {
                Self::MAROON => {
                    __serializer.serialize_field("MAROON", &18446744073701163008u64)
                }
                Self::DARKRED => {
                    __serializer.serialize_field("DARKRED", &18446744073701883904u64)
                }
                Self::RED => {
                    __serializer.serialize_field("RED", &18446744073709486080u64)
                }
                Self::LIGHTPINK => {
                    __serializer.serialize_field("LIGHTPINK", &18446744073709532865u64)
                }
                Self::CRIMSON => {
                    __serializer.serialize_field("CRIMSON", &18446744073707197500u64)
                }
                Self::PALEVIOLETRED => {
                    __serializer
                        .serialize_field("PALEVIOLETRED", &18446744073707155603u64)
                }
                Self::HOTPINK => {
                    __serializer.serialize_field("HOTPINK", &18446744073709513140u64)
                }
                Self::DEEPPINK => {
                    __serializer.serialize_field("DEEPPINK", &18446744073709491347u64)
                }
                Self::MEDIUMVIOLETRED => {
                    __serializer
                        .serialize_field("MEDIUMVIOLETRED", &18446744073705821573u64)
                }
                Self::PURPLE => {
                    __serializer.serialize_field("PURPLE", &18446744073701163136u64)
                }
                Self::DARKMAGENTA => {
                    __serializer.serialize_field("DARKMAGENTA", &18446744073701884043u64)
                }
                Self::ORCHID => {
                    __serializer.serialize_field("ORCHID", &18446744073707090134u64)
                }
                Self::THISTLE => {
                    __serializer.serialize_field("THISTLE", &18446744073706979288u64)
                }
                Self::PLUM => {
                    __serializer.serialize_field("PLUM", &18446744073707299037u64)
                }
                Self::VIOLET => {
                    __serializer.serialize_field("VIOLET", &18446744073708405486u64)
                }
                Self::FUCHSIA => {
                    __serializer.serialize_field("FUCHSIA", &18446744073709486335u64)
                }
                Self::MAGENTA => {
                    __serializer.serialize_field("MAGENTA", &18446744073709486335u64)
                }
                Self::MEDIUMORCHID => {
                    __serializer
                        .serialize_field("MEDIUMORCHID", &18446744073704986067u64)
                }
                Self::DARKVIOLET => {
                    __serializer.serialize_field("DARKVIOLET", &18446744073702473939u64)
                }
                Self::DARKORCHID => {
                    __serializer.serialize_field("DARKORCHID", &18446744073702814412u64)
                }
                Self::BLUEVIOLET => {
                    __serializer.serialize_field("BLUEVIOLET", &18446744073701829602u64)
                }
                Self::INDIGO => {
                    __serializer.serialize_field("INDIGO", &18446744073697689730u64)
                }
                Self::MEDIUMPURPLE => {
                    __serializer
                        .serialize_field("MEDIUMPURPLE", &18446744073702437083u64)
                }
                Self::SLATEBLUE => {
                    __serializer.serialize_field("SLATEBLUE", &18446744073699744461u64)
                }
                Self::MEDIUMSLATEBLUE => {
                    __serializer
                        .serialize_field("MEDIUMSLATEBLUE", &18446744073700862190u64)
                }
                Self::DARKBLUE => {
                    __serializer.serialize_field("DARKBLUE", &18446744073692774539u64)
                }
                Self::MEDIUMBLUE => {
                    __serializer.serialize_field("MEDIUMBLUE", &18446744073692774605u64)
                }
                Self::BLUE => {
                    __serializer.serialize_field("BLUE", &18446744073692774655u64)
                }
                Self::NAVY => {
                    __serializer.serialize_field("NAVY", &18446744073692774528u64)
                }
                Self::MIDNIGHTBLUE => {
                    __serializer
                        .serialize_field("MIDNIGHTBLUE", &18446744073694419312u64)
                }
                Self::DARKSLATEBLUE => {
                    __serializer
                        .serialize_field("DARKSLATEBLUE", &18446744073697508747u64)
                }
                Self::ROYALBLUE => {
                    __serializer.serialize_field("ROYALBLUE", &18446744073697061345u64)
                }
                Self::CORNFLOWERBLUE => {
                    __serializer
                        .serialize_field("CORNFLOWERBLUE", &18446744073699366381u64)
                }
                Self::LIGHTSTEELBLUE => {
                    __serializer
                        .serialize_field("LIGHTSTEELBLUE", &18446744073704359134u64)
                }
                Self::ALICEBLUE => {
                    __serializer.serialize_field("ALICEBLUE", &18446744073708566783u64)
                }
                Self::GHOSTWHITE => {
                    __serializer.serialize_field("GHOSTWHITE", &18446744073709091071u64)
                }
                Self::LAVENDER => {
                    __serializer.serialize_field("LAVENDER", &18446744073707906810u64)
                }
                Self::DODGERBLUE => {
                    __serializer.serialize_field("DODGERBLUE", &18446744073694777599u64)
                }
                Self::STEELBLUE => {
                    __serializer.serialize_field("STEELBLUE", &18446744073697395380u64)
                }
                Self::DEEPSKYBLUE => {
                    __serializer.serialize_field("DEEPSKYBLUE", &18446744073692823551u64)
                }
                Self::SLATEGRAY => {
                    __serializer.serialize_field("SLATEGRAY", &18446744073700147344u64)
                }
                Self::LIGHTSLATEGRAY => {
                    __serializer
                        .serialize_field("LIGHTSLATEGRAY", &18446744073700608153u64)
                }
                Self::LIGHTSKYBLUE => {
                    __serializer
                        .serialize_field("LIGHTSKYBLUE", &18446744073701674746u64)
                }
                Self::SKYBLUE => {
                    __serializer.serialize_field("SKYBLUE", &18446744073701674731u64)
                }
                Self::LIGHTBLUE => {
                    __serializer.serialize_field("LIGHTBLUE", &18446744073704167654u64)
                }
                Self::TEAL => {
                    __serializer.serialize_field("TEAL", &18446744073692807296u64)
                }
                Self::DARKCYAN => {
                    __serializer.serialize_field("DARKCYAN", &18446744073692810123u64)
                }
                Self::DARKTURQUOISE => {
                    __serializer
                        .serialize_field("DARKTURQUOISE", &18446744073692827345u64)
                }
                Self::CYAN => {
                    __serializer.serialize_field("CYAN", &18446744073692839935u64)
                }
                Self::MEDIUMTURQUOISE => {
                    __serializer
                        .serialize_field("MEDIUMTURQUOISE", &18446744073697546700u64)
                }
                Self::CADETBLUE => {
                    __serializer.serialize_field("CADETBLUE", &18446744073699040928u64)
                }
                Self::PALETURQUOISE => {
                    __serializer
                        .serialize_field("PALETURQUOISE", &18446744073704304366u64)
                }
                Self::LIGHTCYAN => {
                    __serializer.serialize_field("LIGHTCYAN", &18446744073707519999u64)
                }
                Self::AZURE => {
                    __serializer.serialize_field("AZURE", &18446744073708568575u64)
                }
                Self::LIGHTSEAGREEN => {
                    __serializer
                        .serialize_field("LIGHTSEAGREEN", &18446744073694917290u64)
                }
                Self::TURQUOISE => {
                    __serializer.serialize_field("TURQUOISE", &18446744073697026256u64)
                }
                Self::POWDERBLUE => {
                    __serializer.serialize_field("POWDERBLUE", &18446744073704366310u64)
                }
                Self::DARKSLATEGRAY => {
                    __serializer
                        .serialize_field("DARKSLATEGRAY", &18446744073695874895u64)
                }
                Self::AQUAMARINE => {
                    __serializer.serialize_field("AQUAMARINE", &18446744073701162964u64)
                }
                Self::MEDIUMSPRINGGREEN => {
                    __serializer
                        .serialize_field("MEDIUMSPRINGGREEN", &18446744073692838554u64)
                }
                Self::MEDIUMAQUAMARINE => {
                    __serializer
                        .serialize_field("MEDIUMAQUAMARINE", &18446744073699511722u64)
                }
                Self::SPRINGGREEN => {
                    __serializer.serialize_field("SPRINGGREEN", &18446744073692839807u64)
                }
                Self::MEDIUMSEAGREEN => {
                    __serializer
                        .serialize_field("MEDIUMSEAGREEN", &18446744073696752497u64)
                }
                Self::SEAGREEN => {
                    __serializer.serialize_field("SEAGREEN", &18446744073695824727u64)
                }
                Self::LIMEGREEN => {
                    __serializer.serialize_field("LIMEGREEN", &18446744073696103730u64)
                }
                Self::DARKGREEN => {
                    __serializer.serialize_field("DARKGREEN", &18446744073692800000u64)
                }
                Self::GREEN => {
                    __serializer.serialize_field("GREEN", &18446744073692807168u64)
                }
                Self::LIME => {
                    __serializer.serialize_field("LIME", &18446744073692839680u64)
                }
                Self::FORESTGREEN => {
                    __serializer.serialize_field("FORESTGREEN", &18446744073695038242u64)
                }
                Self::DARKSEAGREEN => {
                    __serializer
                        .serialize_field("DARKSEAGREEN", &18446744073702194319u64)
                }
                Self::LIGHTGREEN => {
                    __serializer.serialize_field("LIGHTGREEN", &18446744073702272656u64)
                }
                Self::PALEGREEN => {
                    __serializer.serialize_field("PALEGREEN", &18446744073702800280u64)
                }
                Self::MINTCREAM => {
                    __serializer.serialize_field("MINTCREAM", &18446744073708896250u64)
                }
                Self::HONEYDEW => {
                    __serializer.serialize_field("HONEYDEW", &18446744073708568560u64)
                }
                Self::CHARTREUSE => {
                    __serializer.serialize_field("CHARTREUSE", &18446744073701162752u64)
                }
                Self::LAWNGREEN => {
                    __serializer.serialize_field("LAWNGREEN", &18446744073700965376u64)
                }
                Self::OLIVEDRAB => {
                    __serializer.serialize_field("OLIVEDRAB", &18446744073699823139u64)
                }
                Self::DARKOLIVEGREEN => {
                    __serializer
                        .serialize_field("DARKOLIVEGREEN", &18446744073698372399u64)
                }
                Self::YELLOWGREEN => {
                    __serializer.serialize_field("YELLOWGREEN", &18446744073702919474u64)
                }
                Self::GREENYELLOW => {
                    __serializer.serialize_field("GREENYELLOW", &18446744073704177455u64)
                }
                Self::BEIGE => {
                    __serializer.serialize_field("BEIGE", &18446744073708893660u64)
                }
                Self::LINEN => {
                    __serializer.serialize_field("LINEN", &18446744073709220070u64)
                }
                Self::LIGHTGOLDENRODYELLOW => {
                    __serializer
                        .serialize_field(
                            "LIGHTGOLDENRODYELLOW",
                            &18446744073709222610u64,
                        )
                }
                Self::OLIVE => {
                    __serializer.serialize_field("OLIVE", &18446744073701195776u64)
                }
                Self::YELLOW => {
                    __serializer.serialize_field("YELLOW", &18446744073709551360u64)
                }
                Self::LIGHTYELLOW => {
                    __serializer.serialize_field("LIGHTYELLOW", &18446744073709551584u64)
                }
                Self::IVORY => {
                    __serializer.serialize_field("IVORY", &18446744073709551600u64)
                }
                Self::DARKKHAKI => {
                    __serializer.serialize_field("DARKKHAKI", &18446744073705207659u64)
                }
                Self::KHAKI => {
                    __serializer.serialize_field("KHAKI", &18446744073708562060u64)
                }
                Self::PALEGOLDENROD => {
                    __serializer
                        .serialize_field("PALEGOLDENROD", &18446744073708431530u64)
                }
                Self::WHEAT => {
                    __serializer.serialize_field("WHEAT", &18446744073708887731u64)
                }
                Self::GOLD => {
                    __serializer.serialize_field("GOLD", &18446744073709541120u64)
                }
                Self::LEMONCHIFFON => {
                    __serializer
                        .serialize_field("LEMONCHIFFON", &18446744073709550285u64)
                }
                Self::PAPAYAWHIP => {
                    __serializer.serialize_field("PAPAYAWHIP", &18446744073709547477u64)
                }
                Self::DARKGOLDENROD => {
                    __serializer
                        .serialize_field("DARKGOLDENROD", &18446744073704867339u64)
                }
                Self::GOLDENROD => {
                    __serializer.serialize_field("GOLDENROD", &18446744073707103520u64)
                }
                Self::ANTIQUEWHITE => {
                    __serializer
                        .serialize_field("ANTIQUEWHITE", &18446744073709218775u64)
                }
                Self::CORNSILK => {
                    __serializer.serialize_field("CORNSILK", &18446744073709549788u64)
                }
                Self::OLDLACE => {
                    __serializer.serialize_field("OLDLACE", &18446744073709417958u64)
                }
                Self::MOCCASIN => {
                    __serializer.serialize_field("MOCCASIN", &18446744073709544629u64)
                }
                Self::NAVAJOWHITE => {
                    __serializer.serialize_field("NAVAJOWHITE", &18446744073709543085u64)
                }
                Self::ORANGE => {
                    __serializer.serialize_field("ORANGE", &18446744073709528320u64)
                }
                Self::BISQUE => {
                    __serializer.serialize_field("BISQUE", &18446744073709544644u64)
                }
                Self::TAN => {
                    __serializer.serialize_field("TAN", &18446744073706583180u64)
                }
                Self::DARKORANGE => {
                    __serializer.serialize_field("DARKORANGE", &18446744073709521920u64)
                }
                Self::BURLYWOOD => {
                    __serializer.serialize_field("BURLYWOOD", &18446744073707370631u64)
                }
                Self::SADDLEBROWN => {
                    __serializer.serialize_field("SADDLEBROWN", &18446744073701901587u64)
                }
                Self::SANDYBROWN => {
                    __serializer.serialize_field("SANDYBROWN", &18446744073708807264u64)
                }
                Self::BLANCHEDALMOND => {
                    __serializer
                        .serialize_field("BLANCHEDALMOND", &18446744073709546445u64)
                }
                Self::LAVENDERBLUSH => {
                    __serializer
                        .serialize_field("LAVENDERBLUSH", &18446744073709547765u64)
                }
                Self::SEASHELL => {
                    __serializer.serialize_field("SEASHELL", &18446744073709549038u64)
                }
                Self::FLORALWHITE => {
                    __serializer.serialize_field("FLORALWHITE", &18446744073709550320u64)
                }
                Self::SNOW => {
                    __serializer.serialize_field("SNOW", &18446744073709550330u64)
                }
                Self::PERU => {
                    __serializer.serialize_field("PERU", &18446744073706243391u64)
                }
                Self::PEACHPUFF => {
                    __serializer.serialize_field("PEACHPUFF", &18446744073709542073u64)
                }
                Self::CHOCOLATE => {
                    __serializer.serialize_field("CHOCOLATE", &18446744073706563870u64)
                }
                Self::SIENNA => {
                    __serializer.serialize_field("SIENNA", &18446744073703281197u64)
                }
                Self::LIGHTSALMON => {
                    __serializer.serialize_field("LIGHTSALMON", &18446744073709527162u64)
                }
                Self::CORAL => {
                    __serializer.serialize_field("CORAL", &18446744073709518672u64)
                }
                Self::DARKSALMON => {
                    __serializer.serialize_field("DARKSALMON", &18446744073708082810u64)
                }
                Self::MISTYROSE => {
                    __serializer.serialize_field("MISTYROSE", &18446744073709544673u64)
                }
                Self::ORANGERED => {
                    __serializer.serialize_field("ORANGERED", &18446744073709503744u64)
                }
                Self::SALMON => {
                    __serializer.serialize_field("SALMON", &18446744073709191282u64)
                }
                Self::TOMATO => {
                    __serializer.serialize_field("TOMATO", &18446744073709511495u64)
                }
                Self::ROSYBROWN => {
                    __serializer.serialize_field("ROSYBROWN", &18446744073705131919u64)
                }
                Self::PINK => {
                    __serializer.serialize_field("PINK", &18446744073709535435u64)
                }
                Self::INDIANRED => {
                    __serializer.serialize_field("INDIANRED", &18446744073706232924u64)
                }
                Self::LIGHTCORAL => {
                    __serializer.serialize_field("LIGHTCORAL", &18446744073708535936u64)
                }
                Self::BROWN => {
                    __serializer.serialize_field("BROWN", &18446744073703598634u64)
                }
                Self::FIREBRICK => {
                    __serializer.serialize_field("FIREBRICK", &18446744073704448546u64)
                }
                Self::BLACK => {
                    __serializer.serialize_field("BLACK", &18446744073692774400u64)
                }
                Self::DIMGRAY => {
                    __serializer.serialize_field("DIMGRAY", &18446744073699682665u64)
                }
                Self::GRAY => {
                    __serializer.serialize_field("GRAY", &18446744073701195904u64)
                }
                Self::DARKGRAY => {
                    __serializer.serialize_field("DARKGRAY", &18446744073703893417u64)
                }
                Self::SILVER => {
                    __serializer.serialize_field("SILVER", &18446744073705406656u64)
                }
                Self::LIGHTGREY => {
                    __serializer.serialize_field("LIGHTGREY", &18446744073706656723u64)
                }
                Self::GAINSBORO => {
                    __serializer.serialize_field("GAINSBORO", &18446744073707248860u64)
                }
                Self::WHITESMOKE => {
                    __serializer.serialize_field("WHITESMOKE", &18446744073708893685u64)
                }
                Self::WHITE => {
                    __serializer.serialize_field("WHITE", &18446744073709551615u64)
                }
                Self::GREY => {
                    __serializer.serialize_field("GREY", &18446744073701722248u64)
                }
                Self::GREY25 => {
                    __serializer.serialize_field("GREY25", &18446744073696985152u64)
                }
                Self::GREY50 => {
                    __serializer.serialize_field("GREY50", &18446744073701195904u64)
                }
                Self::GREY75 => {
                    __serializer.serialize_field("GREY75", &18446744073705406656u64)
                }
                unknown => {
                    return Err(
                        S::Error::custom(
                            format!(
                                "The enum ExtendedColors contains an unknown value ({}).",
                                unknown.bits()
                            ),
                        ),
                    );
                }
            }?;
            __serializer.serialize_bits(&self.bits())?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ExtendedColors {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
                __field10,
                __field11,
                __field12,
                __field13,
                __field14,
                __field15,
                __field16,
                __field17,
                __field18,
                __field19,
                __field20,
                __field21,
                __field22,
                __field23,
                __field24,
                __field25,
                __field26,
                __field27,
                __field28,
                __field29,
                __field30,
                __field31,
                __field32,
                __field33,
                __field34,
                __field35,
                __field36,
                __field37,
                __field38,
                __field39,
                __field40,
                __field41,
                __field42,
                __field43,
                __field44,
                __field45,
                __field46,
                __field47,
                __field48,
                __field49,
                __field50,
                __field51,
                __field52,
                __field53,
                __field54,
                __field55,
                __field56,
                __field57,
                __field58,
                __field59,
                __field60,
                __field61,
                __field62,
                __field63,
                __field64,
                __field65,
                __field66,
                __field67,
                __field68,
                __field69,
                __field70,
                __field71,
                __field72,
                __field73,
                __field74,
                __field75,
                __field76,
                __field77,
                __field78,
                __field79,
                __field80,
                __field81,
                __field82,
                __field83,
                __field84,
                __field85,
                __field86,
                __field87,
                __field88,
                __field89,
                __field90,
                __field91,
                __field92,
                __field93,
                __field94,
                __field95,
                __field96,
                __field97,
                __field98,
                __field99,
                __field100,
                __field101,
                __field102,
                __field103,
                __field104,
                __field105,
                __field106,
                __field107,
                __field108,
                __field109,
                __field110,
                __field111,
                __field112,
                __field113,
                __field114,
                __field115,
                __field116,
                __field117,
                __field118,
                __field119,
                __field120,
                __field121,
                __field122,
                __field123,
                __field124,
                __field125,
                __field126,
                __field127,
                __field128,
                __field129,
                __field130,
                __field131,
                __field132,
                __field133,
                __field134,
                __field135,
                __field136,
                __field137,
                __field138,
                __field139,
                __field140,
                __field141,
                __field142,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_uint32<__E>(
                    self,
                    __value: U32<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U32::Number(4286578688u32) => {
                            _serde::__private::Ok(__Field::__field0)
                        }
                        U32::Number(4287299584u32) => {
                            _serde::__private::Ok(__Field::__field1)
                        }
                        U32::Number(4294901760u32) => {
                            _serde::__private::Ok(__Field::__field2)
                        }
                        U32::Number(4294948545u32) => {
                            _serde::__private::Ok(__Field::__field3)
                        }
                        U32::Number(4292613180u32) => {
                            _serde::__private::Ok(__Field::__field4)
                        }
                        U32::Number(4292571283u32) => {
                            _serde::__private::Ok(__Field::__field5)
                        }
                        U32::Number(4294928820u32) => {
                            _serde::__private::Ok(__Field::__field6)
                        }
                        U32::Number(4294907027u32) => {
                            _serde::__private::Ok(__Field::__field7)
                        }
                        U32::Number(4291237253u32) => {
                            _serde::__private::Ok(__Field::__field8)
                        }
                        U32::Number(4286578816u32) => {
                            _serde::__private::Ok(__Field::__field9)
                        }
                        U32::Number(4287299723u32) => {
                            _serde::__private::Ok(__Field::__field10)
                        }
                        U32::Number(4292505814u32) => {
                            _serde::__private::Ok(__Field::__field11)
                        }
                        U32::Number(4292394968u32) => {
                            _serde::__private::Ok(__Field::__field12)
                        }
                        U32::Number(4292714717u32) => {
                            _serde::__private::Ok(__Field::__field13)
                        }
                        U32::Number(4293821166u32) => {
                            _serde::__private::Ok(__Field::__field14)
                        }
                        U32::Number(4294902015u32) => {
                            _serde::__private::Ok(__Field::__field15)
                        }
                        U32::Number(4294902015u32) => {
                            _serde::__private::Ok(__Field::__field16)
                        }
                        U32::Number(4290401747u32) => {
                            _serde::__private::Ok(__Field::__field17)
                        }
                        U32::Number(4287889619u32) => {
                            _serde::__private::Ok(__Field::__field18)
                        }
                        U32::Number(4288230092u32) => {
                            _serde::__private::Ok(__Field::__field19)
                        }
                        U32::Number(4287245282u32) => {
                            _serde::__private::Ok(__Field::__field20)
                        }
                        U32::Number(4283105410u32) => {
                            _serde::__private::Ok(__Field::__field21)
                        }
                        U32::Number(4287852763u32) => {
                            _serde::__private::Ok(__Field::__field22)
                        }
                        U32::Number(4285160141u32) => {
                            _serde::__private::Ok(__Field::__field23)
                        }
                        U32::Number(4286277870u32) => {
                            _serde::__private::Ok(__Field::__field24)
                        }
                        U32::Number(4278190219u32) => {
                            _serde::__private::Ok(__Field::__field25)
                        }
                        U32::Number(4278190285u32) => {
                            _serde::__private::Ok(__Field::__field26)
                        }
                        U32::Number(4278190335u32) => {
                            _serde::__private::Ok(__Field::__field27)
                        }
                        U32::Number(4278190208u32) => {
                            _serde::__private::Ok(__Field::__field28)
                        }
                        U32::Number(4279834992u32) => {
                            _serde::__private::Ok(__Field::__field29)
                        }
                        U32::Number(4282924427u32) => {
                            _serde::__private::Ok(__Field::__field30)
                        }
                        U32::Number(4282477025u32) => {
                            _serde::__private::Ok(__Field::__field31)
                        }
                        U32::Number(4284782061u32) => {
                            _serde::__private::Ok(__Field::__field32)
                        }
                        U32::Number(4289774814u32) => {
                            _serde::__private::Ok(__Field::__field33)
                        }
                        U32::Number(4293982463u32) => {
                            _serde::__private::Ok(__Field::__field34)
                        }
                        U32::Number(4294506751u32) => {
                            _serde::__private::Ok(__Field::__field35)
                        }
                        U32::Number(4293322490u32) => {
                            _serde::__private::Ok(__Field::__field36)
                        }
                        U32::Number(4280193279u32) => {
                            _serde::__private::Ok(__Field::__field37)
                        }
                        U32::Number(4282811060u32) => {
                            _serde::__private::Ok(__Field::__field38)
                        }
                        U32::Number(4278239231u32) => {
                            _serde::__private::Ok(__Field::__field39)
                        }
                        U32::Number(4285563024u32) => {
                            _serde::__private::Ok(__Field::__field40)
                        }
                        U32::Number(4286023833u32) => {
                            _serde::__private::Ok(__Field::__field41)
                        }
                        U32::Number(4287090426u32) => {
                            _serde::__private::Ok(__Field::__field42)
                        }
                        U32::Number(4287090411u32) => {
                            _serde::__private::Ok(__Field::__field43)
                        }
                        U32::Number(4289583334u32) => {
                            _serde::__private::Ok(__Field::__field44)
                        }
                        U32::Number(4278222976u32) => {
                            _serde::__private::Ok(__Field::__field45)
                        }
                        U32::Number(4278225803u32) => {
                            _serde::__private::Ok(__Field::__field46)
                        }
                        U32::Number(4278243025u32) => {
                            _serde::__private::Ok(__Field::__field47)
                        }
                        U32::Number(4278255615u32) => {
                            _serde::__private::Ok(__Field::__field48)
                        }
                        U32::Number(4282962380u32) => {
                            _serde::__private::Ok(__Field::__field49)
                        }
                        U32::Number(4284456608u32) => {
                            _serde::__private::Ok(__Field::__field50)
                        }
                        U32::Number(4289720046u32) => {
                            _serde::__private::Ok(__Field::__field51)
                        }
                        U32::Number(4292935679u32) => {
                            _serde::__private::Ok(__Field::__field52)
                        }
                        U32::Number(4293984255u32) => {
                            _serde::__private::Ok(__Field::__field53)
                        }
                        U32::Number(4280332970u32) => {
                            _serde::__private::Ok(__Field::__field54)
                        }
                        U32::Number(4282441936u32) => {
                            _serde::__private::Ok(__Field::__field55)
                        }
                        U32::Number(4289781990u32) => {
                            _serde::__private::Ok(__Field::__field56)
                        }
                        U32::Number(4281290575u32) => {
                            _serde::__private::Ok(__Field::__field57)
                        }
                        U32::Number(4286578644u32) => {
                            _serde::__private::Ok(__Field::__field58)
                        }
                        U32::Number(4278254234u32) => {
                            _serde::__private::Ok(__Field::__field59)
                        }
                        U32::Number(4284927402u32) => {
                            _serde::__private::Ok(__Field::__field60)
                        }
                        U32::Number(4278255487u32) => {
                            _serde::__private::Ok(__Field::__field61)
                        }
                        U32::Number(4282168177u32) => {
                            _serde::__private::Ok(__Field::__field62)
                        }
                        U32::Number(4281240407u32) => {
                            _serde::__private::Ok(__Field::__field63)
                        }
                        U32::Number(4281519410u32) => {
                            _serde::__private::Ok(__Field::__field64)
                        }
                        U32::Number(4278215680u32) => {
                            _serde::__private::Ok(__Field::__field65)
                        }
                        U32::Number(4278222848u32) => {
                            _serde::__private::Ok(__Field::__field66)
                        }
                        U32::Number(4278255360u32) => {
                            _serde::__private::Ok(__Field::__field67)
                        }
                        U32::Number(4280453922u32) => {
                            _serde::__private::Ok(__Field::__field68)
                        }
                        U32::Number(4287609999u32) => {
                            _serde::__private::Ok(__Field::__field69)
                        }
                        U32::Number(4287688336u32) => {
                            _serde::__private::Ok(__Field::__field70)
                        }
                        U32::Number(4288215960u32) => {
                            _serde::__private::Ok(__Field::__field71)
                        }
                        U32::Number(4294311930u32) => {
                            _serde::__private::Ok(__Field::__field72)
                        }
                        U32::Number(4293984240u32) => {
                            _serde::__private::Ok(__Field::__field73)
                        }
                        U32::Number(4286578432u32) => {
                            _serde::__private::Ok(__Field::__field74)
                        }
                        U32::Number(4286381056u32) => {
                            _serde::__private::Ok(__Field::__field75)
                        }
                        U32::Number(4285238819u32) => {
                            _serde::__private::Ok(__Field::__field76)
                        }
                        U32::Number(4283788079u32) => {
                            _serde::__private::Ok(__Field::__field77)
                        }
                        U32::Number(4288335154u32) => {
                            _serde::__private::Ok(__Field::__field78)
                        }
                        U32::Number(4289593135u32) => {
                            _serde::__private::Ok(__Field::__field79)
                        }
                        U32::Number(4294309340u32) => {
                            _serde::__private::Ok(__Field::__field80)
                        }
                        U32::Number(4294635750u32) => {
                            _serde::__private::Ok(__Field::__field81)
                        }
                        U32::Number(4294638290u32) => {
                            _serde::__private::Ok(__Field::__field82)
                        }
                        U32::Number(4286611456u32) => {
                            _serde::__private::Ok(__Field::__field83)
                        }
                        U32::Number(4294967040u32) => {
                            _serde::__private::Ok(__Field::__field84)
                        }
                        U32::Number(4294967264u32) => {
                            _serde::__private::Ok(__Field::__field85)
                        }
                        U32::Number(4294967280u32) => {
                            _serde::__private::Ok(__Field::__field86)
                        }
                        U32::Number(4290623339u32) => {
                            _serde::__private::Ok(__Field::__field87)
                        }
                        U32::Number(4293977740u32) => {
                            _serde::__private::Ok(__Field::__field88)
                        }
                        U32::Number(4293847210u32) => {
                            _serde::__private::Ok(__Field::__field89)
                        }
                        U32::Number(4294303411u32) => {
                            _serde::__private::Ok(__Field::__field90)
                        }
                        U32::Number(4294956800u32) => {
                            _serde::__private::Ok(__Field::__field91)
                        }
                        U32::Number(4294965965u32) => {
                            _serde::__private::Ok(__Field::__field92)
                        }
                        U32::Number(4294963157u32) => {
                            _serde::__private::Ok(__Field::__field93)
                        }
                        U32::Number(4290283019u32) => {
                            _serde::__private::Ok(__Field::__field94)
                        }
                        U32::Number(4292519200u32) => {
                            _serde::__private::Ok(__Field::__field95)
                        }
                        U32::Number(4294634455u32) => {
                            _serde::__private::Ok(__Field::__field96)
                        }
                        U32::Number(4294965468u32) => {
                            _serde::__private::Ok(__Field::__field97)
                        }
                        U32::Number(4294833638u32) => {
                            _serde::__private::Ok(__Field::__field98)
                        }
                        U32::Number(4294960309u32) => {
                            _serde::__private::Ok(__Field::__field99)
                        }
                        U32::Number(4294958765u32) => {
                            _serde::__private::Ok(__Field::__field100)
                        }
                        U32::Number(4294944000u32) => {
                            _serde::__private::Ok(__Field::__field101)
                        }
                        U32::Number(4294960324u32) => {
                            _serde::__private::Ok(__Field::__field102)
                        }
                        U32::Number(4291998860u32) => {
                            _serde::__private::Ok(__Field::__field103)
                        }
                        U32::Number(4294937600u32) => {
                            _serde::__private::Ok(__Field::__field104)
                        }
                        U32::Number(4292786311u32) => {
                            _serde::__private::Ok(__Field::__field105)
                        }
                        U32::Number(4287317267u32) => {
                            _serde::__private::Ok(__Field::__field106)
                        }
                        U32::Number(4294222944u32) => {
                            _serde::__private::Ok(__Field::__field107)
                        }
                        U32::Number(4294962125u32) => {
                            _serde::__private::Ok(__Field::__field108)
                        }
                        U32::Number(4294963445u32) => {
                            _serde::__private::Ok(__Field::__field109)
                        }
                        U32::Number(4294964718u32) => {
                            _serde::__private::Ok(__Field::__field110)
                        }
                        U32::Number(4294966000u32) => {
                            _serde::__private::Ok(__Field::__field111)
                        }
                        U32::Number(4294966010u32) => {
                            _serde::__private::Ok(__Field::__field112)
                        }
                        U32::Number(4291659071u32) => {
                            _serde::__private::Ok(__Field::__field113)
                        }
                        U32::Number(4294957753u32) => {
                            _serde::__private::Ok(__Field::__field114)
                        }
                        U32::Number(4291979550u32) => {
                            _serde::__private::Ok(__Field::__field115)
                        }
                        U32::Number(4288696877u32) => {
                            _serde::__private::Ok(__Field::__field116)
                        }
                        U32::Number(4294942842u32) => {
                            _serde::__private::Ok(__Field::__field117)
                        }
                        U32::Number(4294934352u32) => {
                            _serde::__private::Ok(__Field::__field118)
                        }
                        U32::Number(4293498490u32) => {
                            _serde::__private::Ok(__Field::__field119)
                        }
                        U32::Number(4294960353u32) => {
                            _serde::__private::Ok(__Field::__field120)
                        }
                        U32::Number(4294919424u32) => {
                            _serde::__private::Ok(__Field::__field121)
                        }
                        U32::Number(4294606962u32) => {
                            _serde::__private::Ok(__Field::__field122)
                        }
                        U32::Number(4294927175u32) => {
                            _serde::__private::Ok(__Field::__field123)
                        }
                        U32::Number(4290547599u32) => {
                            _serde::__private::Ok(__Field::__field124)
                        }
                        U32::Number(4294951115u32) => {
                            _serde::__private::Ok(__Field::__field125)
                        }
                        U32::Number(4291648604u32) => {
                            _serde::__private::Ok(__Field::__field126)
                        }
                        U32::Number(4293951616u32) => {
                            _serde::__private::Ok(__Field::__field127)
                        }
                        U32::Number(4289014314u32) => {
                            _serde::__private::Ok(__Field::__field128)
                        }
                        U32::Number(4289864226u32) => {
                            _serde::__private::Ok(__Field::__field129)
                        }
                        U32::Number(4278190080u32) => {
                            _serde::__private::Ok(__Field::__field130)
                        }
                        U32::Number(4285098345u32) => {
                            _serde::__private::Ok(__Field::__field131)
                        }
                        U32::Number(4286611584u32) => {
                            _serde::__private::Ok(__Field::__field132)
                        }
                        U32::Number(4289309097u32) => {
                            _serde::__private::Ok(__Field::__field133)
                        }
                        U32::Number(4290822336u32) => {
                            _serde::__private::Ok(__Field::__field134)
                        }
                        U32::Number(4292072403u32) => {
                            _serde::__private::Ok(__Field::__field135)
                        }
                        U32::Number(4292664540u32) => {
                            _serde::__private::Ok(__Field::__field136)
                        }
                        U32::Number(4294309365u32) => {
                            _serde::__private::Ok(__Field::__field137)
                        }
                        U32::Number(4294967295u32) => {
                            _serde::__private::Ok(__Field::__field138)
                        }
                        U32::Number(4287137928u32) => {
                            _serde::__private::Ok(__Field::__field139)
                        }
                        U32::Number(4282400832u32) => {
                            _serde::__private::Ok(__Field::__field140)
                        }
                        U32::Number(4286611584u32) => {
                            _serde::__private::Ok(__Field::__field141)
                        }
                        U32::Number(4290822336u32) => {
                            _serde::__private::Ok(__Field::__field142)
                        }
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint32(__value),
                                    &"value(u32) of variant is one of -8388608, -7667712, -65536, -18751, -2354116, -2396013, -38476, -60269, -3730043, -8388480, -7667573, -2461482, -2572328, -2252579, -1146130, -65281, -65281, -4565549, -7077677, -6737204, -7722014, -11861886, -7114533, -9807155, -8689426, -16777077, -16777011, -16776961, -16777088, -15132304, -12042869, -12490271, -10185235, -5192482, -984833, -460545, -1644806, -14774017, -12156236, -16728065, -9404272, -8943463, -7876870, -7876885, -5383962, -16744320, -16741493, -16724271, -16711681, -12004916, -10510688, -5247250, -2031617, -983041, -14634326, -12525360, -5185306, -13676721, -8388652, -16713062, -10039894, -16711809, -12799119, -13726889, -13447886, -16751616, -16744448, -16711936, -14513374, -7357297, -7278960, -6751336, -655366, -983056, -8388864, -8586240, -9728477, -11179217, -6632142, -5374161, -657956, -331546, -329006, -8355840, -256, -32, -16, -4343957, -989556, -1120086, -663885, -10496, -1331, -4139, -4684277, -2448096, -332841, -1828, -133658, -6987, -8531, -23296, -6972, -2968436, -29696, -2180985, -7650029, -744352, -5171, -3851, -2578, -1296, -1286, -3308225, -9543, -2987746, -6270419, -24454, -32944, -1468806, -6943, -47872, -360334, -40121, -4419697, -16181, -3318692, -1015680, -5952982, -5103070, -16777216, -9868951, -8355712, -5658199, -4144960, -2894893, -2302756, -657931, -1, -7829368, -12566464, -8355712, -4144960",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "-8388608" || v.eq_ignore_ascii_case("MAROON") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "-7667712" || v.eq_ignore_ascii_case("DARKRED") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "-65536" || v.eq_ignore_ascii_case("RED") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "-18751" || v.eq_ignore_ascii_case("LIGHTPINK") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "-2354116" || v.eq_ignore_ascii_case("CRIMSON") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "-2396013"
                                || v.eq_ignore_ascii_case("PALEVIOLETRED") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "-38476" || v.eq_ignore_ascii_case("HOTPINK") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "-60269" || v.eq_ignore_ascii_case("DEEPPINK") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "-3730043"
                                || v.eq_ignore_ascii_case("MEDIUMVIOLETRED") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "-8388480" || v.eq_ignore_ascii_case("PURPLE") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "-7667573"
                                || v.eq_ignore_ascii_case("DARKMAGENTA") => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            v if v == "-2461482" || v.eq_ignore_ascii_case("ORCHID") => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            v if v == "-2572328" || v.eq_ignore_ascii_case("THISTLE") => {
                                _serde::__private::Ok(__Field::__field12)
                            }
                            v if v == "-2252579" || v.eq_ignore_ascii_case("PLUM") => {
                                _serde::__private::Ok(__Field::__field13)
                            }
                            v if v == "-1146130" || v.eq_ignore_ascii_case("VIOLET") => {
                                _serde::__private::Ok(__Field::__field14)
                            }
                            v if v == "-65281" || v.eq_ignore_ascii_case("FUCHSIA") => {
                                _serde::__private::Ok(__Field::__field15)
                            }
                            v if v == "-65281" || v.eq_ignore_ascii_case("MAGENTA") => {
                                _serde::__private::Ok(__Field::__field16)
                            }
                            v if v == "-4565549"
                                || v.eq_ignore_ascii_case("MEDIUMORCHID") => {
                                _serde::__private::Ok(__Field::__field17)
                            }
                            v if v == "-7077677"
                                || v.eq_ignore_ascii_case("DARKVIOLET") => {
                                _serde::__private::Ok(__Field::__field18)
                            }
                            v if v == "-6737204"
                                || v.eq_ignore_ascii_case("DARKORCHID") => {
                                _serde::__private::Ok(__Field::__field19)
                            }
                            v if v == "-7722014"
                                || v.eq_ignore_ascii_case("BLUEVIOLET") => {
                                _serde::__private::Ok(__Field::__field20)
                            }
                            v if v == "-11861886" || v.eq_ignore_ascii_case("INDIGO") => {
                                _serde::__private::Ok(__Field::__field21)
                            }
                            v if v == "-7114533"
                                || v.eq_ignore_ascii_case("MEDIUMPURPLE") => {
                                _serde::__private::Ok(__Field::__field22)
                            }
                            v if v == "-9807155"
                                || v.eq_ignore_ascii_case("SLATEBLUE") => {
                                _serde::__private::Ok(__Field::__field23)
                            }
                            v if v == "-8689426"
                                || v.eq_ignore_ascii_case("MEDIUMSLATEBLUE") => {
                                _serde::__private::Ok(__Field::__field24)
                            }
                            v if v == "-16777077"
                                || v.eq_ignore_ascii_case("DARKBLUE") => {
                                _serde::__private::Ok(__Field::__field25)
                            }
                            v if v == "-16777011"
                                || v.eq_ignore_ascii_case("MEDIUMBLUE") => {
                                _serde::__private::Ok(__Field::__field26)
                            }
                            v if v == "-16776961" || v.eq_ignore_ascii_case("BLUE") => {
                                _serde::__private::Ok(__Field::__field27)
                            }
                            v if v == "-16777088" || v.eq_ignore_ascii_case("NAVY") => {
                                _serde::__private::Ok(__Field::__field28)
                            }
                            v if v == "-15132304"
                                || v.eq_ignore_ascii_case("MIDNIGHTBLUE") => {
                                _serde::__private::Ok(__Field::__field29)
                            }
                            v if v == "-12042869"
                                || v.eq_ignore_ascii_case("DARKSLATEBLUE") => {
                                _serde::__private::Ok(__Field::__field30)
                            }
                            v if v == "-12490271"
                                || v.eq_ignore_ascii_case("ROYALBLUE") => {
                                _serde::__private::Ok(__Field::__field31)
                            }
                            v if v == "-10185235"
                                || v.eq_ignore_ascii_case("CORNFLOWERBLUE") => {
                                _serde::__private::Ok(__Field::__field32)
                            }
                            v if v == "-5192482"
                                || v.eq_ignore_ascii_case("LIGHTSTEELBLUE") => {
                                _serde::__private::Ok(__Field::__field33)
                            }
                            v if v == "-984833"
                                || v.eq_ignore_ascii_case("ALICEBLUE") => {
                                _serde::__private::Ok(__Field::__field34)
                            }
                            v if v == "-460545"
                                || v.eq_ignore_ascii_case("GHOSTWHITE") => {
                                _serde::__private::Ok(__Field::__field35)
                            }
                            v if v == "-1644806"
                                || v.eq_ignore_ascii_case("LAVENDER") => {
                                _serde::__private::Ok(__Field::__field36)
                            }
                            v if v == "-14774017"
                                || v.eq_ignore_ascii_case("DODGERBLUE") => {
                                _serde::__private::Ok(__Field::__field37)
                            }
                            v if v == "-12156236"
                                || v.eq_ignore_ascii_case("STEELBLUE") => {
                                _serde::__private::Ok(__Field::__field38)
                            }
                            v if v == "-16728065"
                                || v.eq_ignore_ascii_case("DEEPSKYBLUE") => {
                                _serde::__private::Ok(__Field::__field39)
                            }
                            v if v == "-9404272"
                                || v.eq_ignore_ascii_case("SLATEGRAY") => {
                                _serde::__private::Ok(__Field::__field40)
                            }
                            v if v == "-8943463"
                                || v.eq_ignore_ascii_case("LIGHTSLATEGRAY") => {
                                _serde::__private::Ok(__Field::__field41)
                            }
                            v if v == "-7876870"
                                || v.eq_ignore_ascii_case("LIGHTSKYBLUE") => {
                                _serde::__private::Ok(__Field::__field42)
                            }
                            v if v == "-7876885" || v.eq_ignore_ascii_case("SKYBLUE") => {
                                _serde::__private::Ok(__Field::__field43)
                            }
                            v if v == "-5383962"
                                || v.eq_ignore_ascii_case("LIGHTBLUE") => {
                                _serde::__private::Ok(__Field::__field44)
                            }
                            v if v == "-16744320" || v.eq_ignore_ascii_case("TEAL") => {
                                _serde::__private::Ok(__Field::__field45)
                            }
                            v if v == "-16741493"
                                || v.eq_ignore_ascii_case("DARKCYAN") => {
                                _serde::__private::Ok(__Field::__field46)
                            }
                            v if v == "-16724271"
                                || v.eq_ignore_ascii_case("DARKTURQUOISE") => {
                                _serde::__private::Ok(__Field::__field47)
                            }
                            v if v == "-16711681" || v.eq_ignore_ascii_case("CYAN") => {
                                _serde::__private::Ok(__Field::__field48)
                            }
                            v if v == "-12004916"
                                || v.eq_ignore_ascii_case("MEDIUMTURQUOISE") => {
                                _serde::__private::Ok(__Field::__field49)
                            }
                            v if v == "-10510688"
                                || v.eq_ignore_ascii_case("CADETBLUE") => {
                                _serde::__private::Ok(__Field::__field50)
                            }
                            v if v == "-5247250"
                                || v.eq_ignore_ascii_case("PALETURQUOISE") => {
                                _serde::__private::Ok(__Field::__field51)
                            }
                            v if v == "-2031617"
                                || v.eq_ignore_ascii_case("LIGHTCYAN") => {
                                _serde::__private::Ok(__Field::__field52)
                            }
                            v if v == "-983041" || v.eq_ignore_ascii_case("AZURE") => {
                                _serde::__private::Ok(__Field::__field53)
                            }
                            v if v == "-14634326"
                                || v.eq_ignore_ascii_case("LIGHTSEAGREEN") => {
                                _serde::__private::Ok(__Field::__field54)
                            }
                            v if v == "-12525360"
                                || v.eq_ignore_ascii_case("TURQUOISE") => {
                                _serde::__private::Ok(__Field::__field55)
                            }
                            v if v == "-5185306"
                                || v.eq_ignore_ascii_case("POWDERBLUE") => {
                                _serde::__private::Ok(__Field::__field56)
                            }
                            v if v == "-13676721"
                                || v.eq_ignore_ascii_case("DARKSLATEGRAY") => {
                                _serde::__private::Ok(__Field::__field57)
                            }
                            v if v == "-8388652"
                                || v.eq_ignore_ascii_case("AQUAMARINE") => {
                                _serde::__private::Ok(__Field::__field58)
                            }
                            v if v == "-16713062"
                                || v.eq_ignore_ascii_case("MEDIUMSPRINGGREEN") => {
                                _serde::__private::Ok(__Field::__field59)
                            }
                            v if v == "-10039894"
                                || v.eq_ignore_ascii_case("MEDIUMAQUAMARINE") => {
                                _serde::__private::Ok(__Field::__field60)
                            }
                            v if v == "-16711809"
                                || v.eq_ignore_ascii_case("SPRINGGREEN") => {
                                _serde::__private::Ok(__Field::__field61)
                            }
                            v if v == "-12799119"
                                || v.eq_ignore_ascii_case("MEDIUMSEAGREEN") => {
                                _serde::__private::Ok(__Field::__field62)
                            }
                            v if v == "-13726889"
                                || v.eq_ignore_ascii_case("SEAGREEN") => {
                                _serde::__private::Ok(__Field::__field63)
                            }
                            v if v == "-13447886"
                                || v.eq_ignore_ascii_case("LIMEGREEN") => {
                                _serde::__private::Ok(__Field::__field64)
                            }
                            v if v == "-16751616"
                                || v.eq_ignore_ascii_case("DARKGREEN") => {
                                _serde::__private::Ok(__Field::__field65)
                            }
                            v if v == "-16744448" || v.eq_ignore_ascii_case("GREEN") => {
                                _serde::__private::Ok(__Field::__field66)
                            }
                            v if v == "-16711936" || v.eq_ignore_ascii_case("LIME") => {
                                _serde::__private::Ok(__Field::__field67)
                            }
                            v if v == "-14513374"
                                || v.eq_ignore_ascii_case("FORESTGREEN") => {
                                _serde::__private::Ok(__Field::__field68)
                            }
                            v if v == "-7357297"
                                || v.eq_ignore_ascii_case("DARKSEAGREEN") => {
                                _serde::__private::Ok(__Field::__field69)
                            }
                            v if v == "-7278960"
                                || v.eq_ignore_ascii_case("LIGHTGREEN") => {
                                _serde::__private::Ok(__Field::__field70)
                            }
                            v if v == "-6751336"
                                || v.eq_ignore_ascii_case("PALEGREEN") => {
                                _serde::__private::Ok(__Field::__field71)
                            }
                            v if v == "-655366"
                                || v.eq_ignore_ascii_case("MINTCREAM") => {
                                _serde::__private::Ok(__Field::__field72)
                            }
                            v if v == "-983056" || v.eq_ignore_ascii_case("HONEYDEW") => {
                                _serde::__private::Ok(__Field::__field73)
                            }
                            v if v == "-8388864"
                                || v.eq_ignore_ascii_case("CHARTREUSE") => {
                                _serde::__private::Ok(__Field::__field74)
                            }
                            v if v == "-8586240"
                                || v.eq_ignore_ascii_case("LAWNGREEN") => {
                                _serde::__private::Ok(__Field::__field75)
                            }
                            v if v == "-9728477"
                                || v.eq_ignore_ascii_case("OLIVEDRAB") => {
                                _serde::__private::Ok(__Field::__field76)
                            }
                            v if v == "-11179217"
                                || v.eq_ignore_ascii_case("DARKOLIVEGREEN") => {
                                _serde::__private::Ok(__Field::__field77)
                            }
                            v if v == "-6632142"
                                || v.eq_ignore_ascii_case("YELLOWGREEN") => {
                                _serde::__private::Ok(__Field::__field78)
                            }
                            v if v == "-5374161"
                                || v.eq_ignore_ascii_case("GREENYELLOW") => {
                                _serde::__private::Ok(__Field::__field79)
                            }
                            v if v == "-657956" || v.eq_ignore_ascii_case("BEIGE") => {
                                _serde::__private::Ok(__Field::__field80)
                            }
                            v if v == "-331546" || v.eq_ignore_ascii_case("LINEN") => {
                                _serde::__private::Ok(__Field::__field81)
                            }
                            v if v == "-329006"
                                || v.eq_ignore_ascii_case("LIGHTGOLDENRODYELLOW") => {
                                _serde::__private::Ok(__Field::__field82)
                            }
                            v if v == "-8355840" || v.eq_ignore_ascii_case("OLIVE") => {
                                _serde::__private::Ok(__Field::__field83)
                            }
                            v if v == "-256" || v.eq_ignore_ascii_case("YELLOW") => {
                                _serde::__private::Ok(__Field::__field84)
                            }
                            v if v == "-32" || v.eq_ignore_ascii_case("LIGHTYELLOW") => {
                                _serde::__private::Ok(__Field::__field85)
                            }
                            v if v == "-16" || v.eq_ignore_ascii_case("IVORY") => {
                                _serde::__private::Ok(__Field::__field86)
                            }
                            v if v == "-4343957"
                                || v.eq_ignore_ascii_case("DARKKHAKI") => {
                                _serde::__private::Ok(__Field::__field87)
                            }
                            v if v == "-989556" || v.eq_ignore_ascii_case("KHAKI") => {
                                _serde::__private::Ok(__Field::__field88)
                            }
                            v if v == "-1120086"
                                || v.eq_ignore_ascii_case("PALEGOLDENROD") => {
                                _serde::__private::Ok(__Field::__field89)
                            }
                            v if v == "-663885" || v.eq_ignore_ascii_case("WHEAT") => {
                                _serde::__private::Ok(__Field::__field90)
                            }
                            v if v == "-10496" || v.eq_ignore_ascii_case("GOLD") => {
                                _serde::__private::Ok(__Field::__field91)
                            }
                            v if v == "-1331"
                                || v.eq_ignore_ascii_case("LEMONCHIFFON") => {
                                _serde::__private::Ok(__Field::__field92)
                            }
                            v if v == "-4139" || v.eq_ignore_ascii_case("PAPAYAWHIP") => {
                                _serde::__private::Ok(__Field::__field93)
                            }
                            v if v == "-4684277"
                                || v.eq_ignore_ascii_case("DARKGOLDENROD") => {
                                _serde::__private::Ok(__Field::__field94)
                            }
                            v if v == "-2448096"
                                || v.eq_ignore_ascii_case("GOLDENROD") => {
                                _serde::__private::Ok(__Field::__field95)
                            }
                            v if v == "-332841"
                                || v.eq_ignore_ascii_case("ANTIQUEWHITE") => {
                                _serde::__private::Ok(__Field::__field96)
                            }
                            v if v == "-1828" || v.eq_ignore_ascii_case("CORNSILK") => {
                                _serde::__private::Ok(__Field::__field97)
                            }
                            v if v == "-133658" || v.eq_ignore_ascii_case("OLDLACE") => {
                                _serde::__private::Ok(__Field::__field98)
                            }
                            v if v == "-6987" || v.eq_ignore_ascii_case("MOCCASIN") => {
                                _serde::__private::Ok(__Field::__field99)
                            }
                            v if v == "-8531"
                                || v.eq_ignore_ascii_case("NAVAJOWHITE") => {
                                _serde::__private::Ok(__Field::__field100)
                            }
                            v if v == "-23296" || v.eq_ignore_ascii_case("ORANGE") => {
                                _serde::__private::Ok(__Field::__field101)
                            }
                            v if v == "-6972" || v.eq_ignore_ascii_case("BISQUE") => {
                                _serde::__private::Ok(__Field::__field102)
                            }
                            v if v == "-2968436" || v.eq_ignore_ascii_case("TAN") => {
                                _serde::__private::Ok(__Field::__field103)
                            }
                            v if v == "-29696"
                                || v.eq_ignore_ascii_case("DARKORANGE") => {
                                _serde::__private::Ok(__Field::__field104)
                            }
                            v if v == "-2180985"
                                || v.eq_ignore_ascii_case("BURLYWOOD") => {
                                _serde::__private::Ok(__Field::__field105)
                            }
                            v if v == "-7650029"
                                || v.eq_ignore_ascii_case("SADDLEBROWN") => {
                                _serde::__private::Ok(__Field::__field106)
                            }
                            v if v == "-744352"
                                || v.eq_ignore_ascii_case("SANDYBROWN") => {
                                _serde::__private::Ok(__Field::__field107)
                            }
                            v if v == "-5171"
                                || v.eq_ignore_ascii_case("BLANCHEDALMOND") => {
                                _serde::__private::Ok(__Field::__field108)
                            }
                            v if v == "-3851"
                                || v.eq_ignore_ascii_case("LAVENDERBLUSH") => {
                                _serde::__private::Ok(__Field::__field109)
                            }
                            v if v == "-2578" || v.eq_ignore_ascii_case("SEASHELL") => {
                                _serde::__private::Ok(__Field::__field110)
                            }
                            v if v == "-1296"
                                || v.eq_ignore_ascii_case("FLORALWHITE") => {
                                _serde::__private::Ok(__Field::__field111)
                            }
                            v if v == "-1286" || v.eq_ignore_ascii_case("SNOW") => {
                                _serde::__private::Ok(__Field::__field112)
                            }
                            v if v == "-3308225" || v.eq_ignore_ascii_case("PERU") => {
                                _serde::__private::Ok(__Field::__field113)
                            }
                            v if v == "-9543" || v.eq_ignore_ascii_case("PEACHPUFF") => {
                                _serde::__private::Ok(__Field::__field114)
                            }
                            v if v == "-2987746"
                                || v.eq_ignore_ascii_case("CHOCOLATE") => {
                                _serde::__private::Ok(__Field::__field115)
                            }
                            v if v == "-6270419" || v.eq_ignore_ascii_case("SIENNA") => {
                                _serde::__private::Ok(__Field::__field116)
                            }
                            v if v == "-24454"
                                || v.eq_ignore_ascii_case("LIGHTSALMON") => {
                                _serde::__private::Ok(__Field::__field117)
                            }
                            v if v == "-32944" || v.eq_ignore_ascii_case("CORAL") => {
                                _serde::__private::Ok(__Field::__field118)
                            }
                            v if v == "-1468806"
                                || v.eq_ignore_ascii_case("DARKSALMON") => {
                                _serde::__private::Ok(__Field::__field119)
                            }
                            v if v == "-6943" || v.eq_ignore_ascii_case("MISTYROSE") => {
                                _serde::__private::Ok(__Field::__field120)
                            }
                            v if v == "-47872" || v.eq_ignore_ascii_case("ORANGERED") => {
                                _serde::__private::Ok(__Field::__field121)
                            }
                            v if v == "-360334" || v.eq_ignore_ascii_case("SALMON") => {
                                _serde::__private::Ok(__Field::__field122)
                            }
                            v if v == "-40121" || v.eq_ignore_ascii_case("TOMATO") => {
                                _serde::__private::Ok(__Field::__field123)
                            }
                            v if v == "-4419697"
                                || v.eq_ignore_ascii_case("ROSYBROWN") => {
                                _serde::__private::Ok(__Field::__field124)
                            }
                            v if v == "-16181" || v.eq_ignore_ascii_case("PINK") => {
                                _serde::__private::Ok(__Field::__field125)
                            }
                            v if v == "-3318692"
                                || v.eq_ignore_ascii_case("INDIANRED") => {
                                _serde::__private::Ok(__Field::__field126)
                            }
                            v if v == "-1015680"
                                || v.eq_ignore_ascii_case("LIGHTCORAL") => {
                                _serde::__private::Ok(__Field::__field127)
                            }
                            v if v == "-5952982" || v.eq_ignore_ascii_case("BROWN") => {
                                _serde::__private::Ok(__Field::__field128)
                            }
                            v if v == "-5103070"
                                || v.eq_ignore_ascii_case("FIREBRICK") => {
                                _serde::__private::Ok(__Field::__field129)
                            }
                            v if v == "-16777216" || v.eq_ignore_ascii_case("BLACK") => {
                                _serde::__private::Ok(__Field::__field130)
                            }
                            v if v == "-9868951" || v.eq_ignore_ascii_case("DIMGRAY") => {
                                _serde::__private::Ok(__Field::__field131)
                            }
                            v if v == "-8355712" || v.eq_ignore_ascii_case("GRAY") => {
                                _serde::__private::Ok(__Field::__field132)
                            }
                            v if v == "-5658199"
                                || v.eq_ignore_ascii_case("DARKGRAY") => {
                                _serde::__private::Ok(__Field::__field133)
                            }
                            v if v == "-4144960" || v.eq_ignore_ascii_case("SILVER") => {
                                _serde::__private::Ok(__Field::__field134)
                            }
                            v if v == "-2894893"
                                || v.eq_ignore_ascii_case("LIGHTGREY") => {
                                _serde::__private::Ok(__Field::__field135)
                            }
                            v if v == "-2302756"
                                || v.eq_ignore_ascii_case("GAINSBORO") => {
                                _serde::__private::Ok(__Field::__field136)
                            }
                            v if v == "-657931"
                                || v.eq_ignore_ascii_case("WHITESMOKE") => {
                                _serde::__private::Ok(__Field::__field137)
                            }
                            v if v == "-1" || v.eq_ignore_ascii_case("WHITE") => {
                                _serde::__private::Ok(__Field::__field138)
                            }
                            v if v == "-7829368" || v.eq_ignore_ascii_case("GREY") => {
                                _serde::__private::Ok(__Field::__field139)
                            }
                            v if v == "-12566464" || v.eq_ignore_ascii_case("GREY25") => {
                                _serde::__private::Ok(__Field::__field140)
                            }
                            v if v == "-8355712" || v.eq_ignore_ascii_case("GREY50") => {
                                _serde::__private::Ok(__Field::__field141)
                            }
                            v if v == "-4144960" || v.eq_ignore_ascii_case("GREY75") => {
                                _serde::__private::Ok(__Field::__field142)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Uint32,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<ExtendedColors>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ExtendedColors;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum ExtendedColors",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MAROON)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKRED)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::RED)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIGHTPINK)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::CRIMSON)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::PALEVIOLETRED)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::HOTPINK)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DEEPPINK)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MEDIUMVIOLETRED)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::PURPLE)
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKMAGENTA)
                        }
                        (__Field::__field11, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::ORCHID)
                        }
                        (__Field::__field12, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::THISTLE)
                        }
                        (__Field::__field13, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::PLUM)
                        }
                        (__Field::__field14, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::VIOLET)
                        }
                        (__Field::__field15, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::FUCHSIA)
                        }
                        (__Field::__field16, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MAGENTA)
                        }
                        (__Field::__field17, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MEDIUMORCHID)
                        }
                        (__Field::__field18, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKVIOLET)
                        }
                        (__Field::__field19, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKORCHID)
                        }
                        (__Field::__field20, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::BLUEVIOLET)
                        }
                        (__Field::__field21, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::INDIGO)
                        }
                        (__Field::__field22, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MEDIUMPURPLE)
                        }
                        (__Field::__field23, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::SLATEBLUE)
                        }
                        (__Field::__field24, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MEDIUMSLATEBLUE)
                        }
                        (__Field::__field25, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKBLUE)
                        }
                        (__Field::__field26, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MEDIUMBLUE)
                        }
                        (__Field::__field27, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::BLUE)
                        }
                        (__Field::__field28, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::NAVY)
                        }
                        (__Field::__field29, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MIDNIGHTBLUE)
                        }
                        (__Field::__field30, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKSLATEBLUE)
                        }
                        (__Field::__field31, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::ROYALBLUE)
                        }
                        (__Field::__field32, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::CORNFLOWERBLUE)
                        }
                        (__Field::__field33, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIGHTSTEELBLUE)
                        }
                        (__Field::__field34, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::ALICEBLUE)
                        }
                        (__Field::__field35, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::GHOSTWHITE)
                        }
                        (__Field::__field36, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LAVENDER)
                        }
                        (__Field::__field37, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DODGERBLUE)
                        }
                        (__Field::__field38, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::STEELBLUE)
                        }
                        (__Field::__field39, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DEEPSKYBLUE)
                        }
                        (__Field::__field40, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::SLATEGRAY)
                        }
                        (__Field::__field41, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIGHTSLATEGRAY)
                        }
                        (__Field::__field42, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIGHTSKYBLUE)
                        }
                        (__Field::__field43, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::SKYBLUE)
                        }
                        (__Field::__field44, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIGHTBLUE)
                        }
                        (__Field::__field45, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::TEAL)
                        }
                        (__Field::__field46, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKCYAN)
                        }
                        (__Field::__field47, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKTURQUOISE)
                        }
                        (__Field::__field48, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::CYAN)
                        }
                        (__Field::__field49, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MEDIUMTURQUOISE)
                        }
                        (__Field::__field50, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::CADETBLUE)
                        }
                        (__Field::__field51, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::PALETURQUOISE)
                        }
                        (__Field::__field52, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIGHTCYAN)
                        }
                        (__Field::__field53, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::AZURE)
                        }
                        (__Field::__field54, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIGHTSEAGREEN)
                        }
                        (__Field::__field55, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::TURQUOISE)
                        }
                        (__Field::__field56, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::POWDERBLUE)
                        }
                        (__Field::__field57, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKSLATEGRAY)
                        }
                        (__Field::__field58, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::AQUAMARINE)
                        }
                        (__Field::__field59, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MEDIUMSPRINGGREEN)
                        }
                        (__Field::__field60, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MEDIUMAQUAMARINE)
                        }
                        (__Field::__field61, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::SPRINGGREEN)
                        }
                        (__Field::__field62, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MEDIUMSEAGREEN)
                        }
                        (__Field::__field63, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::SEAGREEN)
                        }
                        (__Field::__field64, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIMEGREEN)
                        }
                        (__Field::__field65, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKGREEN)
                        }
                        (__Field::__field66, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::GREEN)
                        }
                        (__Field::__field67, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIME)
                        }
                        (__Field::__field68, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::FORESTGREEN)
                        }
                        (__Field::__field69, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKSEAGREEN)
                        }
                        (__Field::__field70, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIGHTGREEN)
                        }
                        (__Field::__field71, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::PALEGREEN)
                        }
                        (__Field::__field72, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MINTCREAM)
                        }
                        (__Field::__field73, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::HONEYDEW)
                        }
                        (__Field::__field74, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::CHARTREUSE)
                        }
                        (__Field::__field75, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LAWNGREEN)
                        }
                        (__Field::__field76, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::OLIVEDRAB)
                        }
                        (__Field::__field77, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKOLIVEGREEN)
                        }
                        (__Field::__field78, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::YELLOWGREEN)
                        }
                        (__Field::__field79, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::GREENYELLOW)
                        }
                        (__Field::__field80, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::BEIGE)
                        }
                        (__Field::__field81, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LINEN)
                        }
                        (__Field::__field82, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIGHTGOLDENRODYELLOW)
                        }
                        (__Field::__field83, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::OLIVE)
                        }
                        (__Field::__field84, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::YELLOW)
                        }
                        (__Field::__field85, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIGHTYELLOW)
                        }
                        (__Field::__field86, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::IVORY)
                        }
                        (__Field::__field87, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKKHAKI)
                        }
                        (__Field::__field88, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::KHAKI)
                        }
                        (__Field::__field89, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::PALEGOLDENROD)
                        }
                        (__Field::__field90, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::WHEAT)
                        }
                        (__Field::__field91, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::GOLD)
                        }
                        (__Field::__field92, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LEMONCHIFFON)
                        }
                        (__Field::__field93, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::PAPAYAWHIP)
                        }
                        (__Field::__field94, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKGOLDENROD)
                        }
                        (__Field::__field95, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::GOLDENROD)
                        }
                        (__Field::__field96, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::ANTIQUEWHITE)
                        }
                        (__Field::__field97, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::CORNSILK)
                        }
                        (__Field::__field98, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::OLDLACE)
                        }
                        (__Field::__field99, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MOCCASIN)
                        }
                        (__Field::__field100, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::NAVAJOWHITE)
                        }
                        (__Field::__field101, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::ORANGE)
                        }
                        (__Field::__field102, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::BISQUE)
                        }
                        (__Field::__field103, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::TAN)
                        }
                        (__Field::__field104, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKORANGE)
                        }
                        (__Field::__field105, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::BURLYWOOD)
                        }
                        (__Field::__field106, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::SADDLEBROWN)
                        }
                        (__Field::__field107, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::SANDYBROWN)
                        }
                        (__Field::__field108, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::BLANCHEDALMOND)
                        }
                        (__Field::__field109, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LAVENDERBLUSH)
                        }
                        (__Field::__field110, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::SEASHELL)
                        }
                        (__Field::__field111, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::FLORALWHITE)
                        }
                        (__Field::__field112, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::SNOW)
                        }
                        (__Field::__field113, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::PERU)
                        }
                        (__Field::__field114, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::PEACHPUFF)
                        }
                        (__Field::__field115, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::CHOCOLATE)
                        }
                        (__Field::__field116, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::SIENNA)
                        }
                        (__Field::__field117, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIGHTSALMON)
                        }
                        (__Field::__field118, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::CORAL)
                        }
                        (__Field::__field119, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKSALMON)
                        }
                        (__Field::__field120, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::MISTYROSE)
                        }
                        (__Field::__field121, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::ORANGERED)
                        }
                        (__Field::__field122, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::SALMON)
                        }
                        (__Field::__field123, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::TOMATO)
                        }
                        (__Field::__field124, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::ROSYBROWN)
                        }
                        (__Field::__field125, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::PINK)
                        }
                        (__Field::__field126, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::INDIANRED)
                        }
                        (__Field::__field127, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIGHTCORAL)
                        }
                        (__Field::__field128, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::BROWN)
                        }
                        (__Field::__field129, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::FIREBRICK)
                        }
                        (__Field::__field130, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::BLACK)
                        }
                        (__Field::__field131, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DIMGRAY)
                        }
                        (__Field::__field132, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::GRAY)
                        }
                        (__Field::__field133, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::DARKGRAY)
                        }
                        (__Field::__field134, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::SILVER)
                        }
                        (__Field::__field135, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::LIGHTGREY)
                        }
                        (__Field::__field136, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::GAINSBORO)
                        }
                        (__Field::__field137, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::WHITESMOKE)
                        }
                        (__Field::__field138, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::WHITE)
                        }
                        (__Field::__field139, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::GREY)
                        }
                        (__Field::__field140, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::GREY25)
                        }
                        (__Field::__field141, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::GREY50)
                        }
                        (__Field::__field142, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ExtendedColors::GREY75)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "MAROON",
                "DARKRED",
                "RED",
                "LIGHTPINK",
                "CRIMSON",
                "PALEVIOLETRED",
                "HOTPINK",
                "DEEPPINK",
                "MEDIUMVIOLETRED",
                "PURPLE",
                "DARKMAGENTA",
                "ORCHID",
                "THISTLE",
                "PLUM",
                "VIOLET",
                "FUCHSIA",
                "MAGENTA",
                "MEDIUMORCHID",
                "DARKVIOLET",
                "DARKORCHID",
                "BLUEVIOLET",
                "INDIGO",
                "MEDIUMPURPLE",
                "SLATEBLUE",
                "MEDIUMSLATEBLUE",
                "DARKBLUE",
                "MEDIUMBLUE",
                "BLUE",
                "NAVY",
                "MIDNIGHTBLUE",
                "DARKSLATEBLUE",
                "ROYALBLUE",
                "CORNFLOWERBLUE",
                "LIGHTSTEELBLUE",
                "ALICEBLUE",
                "GHOSTWHITE",
                "LAVENDER",
                "DODGERBLUE",
                "STEELBLUE",
                "DEEPSKYBLUE",
                "SLATEGRAY",
                "LIGHTSLATEGRAY",
                "LIGHTSKYBLUE",
                "SKYBLUE",
                "LIGHTBLUE",
                "TEAL",
                "DARKCYAN",
                "DARKTURQUOISE",
                "CYAN",
                "MEDIUMTURQUOISE",
                "CADETBLUE",
                "PALETURQUOISE",
                "LIGHTCYAN",
                "AZURE",
                "LIGHTSEAGREEN",
                "TURQUOISE",
                "POWDERBLUE",
                "DARKSLATEGRAY",
                "AQUAMARINE",
                "MEDIUMSPRINGGREEN",
                "MEDIUMAQUAMARINE",
                "SPRINGGREEN",
                "MEDIUMSEAGREEN",
                "SEAGREEN",
                "LIMEGREEN",
                "DARKGREEN",
                "GREEN",
                "LIME",
                "FORESTGREEN",
                "DARKSEAGREEN",
                "LIGHTGREEN",
                "PALEGREEN",
                "MINTCREAM",
                "HONEYDEW",
                "CHARTREUSE",
                "LAWNGREEN",
                "OLIVEDRAB",
                "DARKOLIVEGREEN",
                "YELLOWGREEN",
                "GREENYELLOW",
                "BEIGE",
                "LINEN",
                "LIGHTGOLDENRODYELLOW",
                "OLIVE",
                "YELLOW",
                "LIGHTYELLOW",
                "IVORY",
                "DARKKHAKI",
                "KHAKI",
                "PALEGOLDENROD",
                "WHEAT",
                "GOLD",
                "LEMONCHIFFON",
                "PAPAYAWHIP",
                "DARKGOLDENROD",
                "GOLDENROD",
                "ANTIQUEWHITE",
                "CORNSILK",
                "OLDLACE",
                "MOCCASIN",
                "NAVAJOWHITE",
                "ORANGE",
                "BISQUE",
                "TAN",
                "DARKORANGE",
                "BURLYWOOD",
                "SADDLEBROWN",
                "SANDYBROWN",
                "BLANCHEDALMOND",
                "LAVENDERBLUSH",
                "SEASHELL",
                "FLORALWHITE",
                "SNOW",
                "PERU",
                "PEACHPUFF",
                "CHOCOLATE",
                "SIENNA",
                "LIGHTSALMON",
                "CORAL",
                "DARKSALMON",
                "MISTYROSE",
                "ORANGERED",
                "SALMON",
                "TOMATO",
                "ROSYBROWN",
                "PINK",
                "INDIANRED",
                "LIGHTCORAL",
                "BROWN",
                "FIREBRICK",
                "BLACK",
                "DIMGRAY",
                "GRAY",
                "DARKGRAY",
                "SILVER",
                "LIGHTGREY",
                "GAINSBORO",
                "WHITESMOKE",
                "WHITE",
                "GREY",
                "GREY25",
                "GREY50",
                "GREY75",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "ExtendedColors",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ExtendedColors>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
