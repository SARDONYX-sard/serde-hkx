use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkColor`
/// -         version: `0`
/// -       signature: `0x106b96ce`
/// -          size:   1(x86)/  1(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkColor {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkColor {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkColor"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(275486414u32)
        }
    }
    impl __serde::Serialize for hkColor {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer.serialize_struct("hkColor", class_meta)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.end()
        }
    }
};
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkEnum<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_UINT32`"] #[doc = r""] #[doc =
    r" # Why this `enum` defined as `bitflags`?"] #[doc =
    r" These are not really `TYPE_FLAGS` but `TYPE_ENUM`, but since Rust does not allow the definition of"]
    #[doc =
    r" `enum` with duplicate discriminant values, they are defined as `bitflags`."]
    #[allow(non_upper_case_globals, non_snake_case)] #[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize))] #[repr(transparent)] #[derive(Debug,
    Clone, Copy, Default, PartialEq, Eq, Hash)] pub struct ExtendedColors : u32 { #[doc =
    "-8388608"] const MAROON = 4286578688u32; #[doc = "-7667712"] const DARKRED =
    4287299584u32; #[doc = "-65536"] const RED = 4294901760u32; #[doc = "-18751"] const
    LIGHTPINK = 4294948545u32; #[doc = "-2354116"] const CRIMSON = 4292613180u32; #[doc =
    "-2396013"] const PALEVIOLETRED = 4292571283u32; #[doc = "-38476"] const HOTPINK =
    4294928820u32; #[doc = "-60269"] const DEEPPINK = 4294907027u32; #[doc = "-3730043"]
    const MEDIUMVIOLETRED = 4291237253u32; #[doc = "-8388480"] const PURPLE =
    4286578816u32; #[doc = "-7667573"] const DARKMAGENTA = 4287299723u32; #[doc =
    "-2461482"] const ORCHID = 4292505814u32; #[doc = "-2572328"] const THISTLE =
    4292394968u32; #[doc = "-2252579"] const PLUM = 4292714717u32; #[doc = "-1146130"]
    const VIOLET = 4293821166u32; #[doc = "-65281"] const FUCHSIA = 4294902015u32; #[doc
    = "-65281"] const MAGENTA = 4294902015u32; #[doc = "-4565549"] const MEDIUMORCHID =
    4290401747u32; #[doc = "-7077677"] const DARKVIOLET = 4287889619u32; #[doc =
    "-6737204"] const DARKORCHID = 4288230092u32; #[doc = "-7722014"] const BLUEVIOLET =
    4287245282u32; #[doc = "-11861886"] const INDIGO = 4283105410u32; #[doc = "-7114533"]
    const MEDIUMPURPLE = 4287852763u32; #[doc = "-9807155"] const SLATEBLUE =
    4285160141u32; #[doc = "-8689426"] const MEDIUMSLATEBLUE = 4286277870u32; #[doc =
    "-16777077"] const DARKBLUE = 4278190219u32; #[doc = "-16777011"] const MEDIUMBLUE =
    4278190285u32; #[doc = "-16776961"] const BLUE = 4278190335u32; #[doc = "-16777088"]
    const NAVY = 4278190208u32; #[doc = "-15132304"] const MIDNIGHTBLUE = 4279834992u32;
    #[doc = "-12042869"] const DARKSLATEBLUE = 4282924427u32; #[doc = "-12490271"] const
    ROYALBLUE = 4282477025u32; #[doc = "-10185235"] const CORNFLOWERBLUE = 4284782061u32;
    #[doc = "-5192482"] const LIGHTSTEELBLUE = 4289774814u32; #[doc = "-984833"] const
    ALICEBLUE = 4293982463u32; #[doc = "-460545"] const GHOSTWHITE = 4294506751u32; #[doc
    = "-1644806"] const LAVENDER = 4293322490u32; #[doc = "-14774017"] const DODGERBLUE =
    4280193279u32; #[doc = "-12156236"] const STEELBLUE = 4282811060u32; #[doc =
    "-16728065"] const DEEPSKYBLUE = 4278239231u32; #[doc = "-9404272"] const SLATEGRAY =
    4285563024u32; #[doc = "-8943463"] const LIGHTSLATEGRAY = 4286023833u32; #[doc =
    "-7876870"] const LIGHTSKYBLUE = 4287090426u32; #[doc = "-7876885"] const SKYBLUE =
    4287090411u32; #[doc = "-5383962"] const LIGHTBLUE = 4289583334u32; #[doc =
    "-16744320"] const TEAL = 4278222976u32; #[doc = "-16741493"] const DARKCYAN =
    4278225803u32; #[doc = "-16724271"] const DARKTURQUOISE = 4278243025u32; #[doc =
    "-16711681"] const CYAN = 4278255615u32; #[doc = "-12004916"] const MEDIUMTURQUOISE =
    4282962380u32; #[doc = "-10510688"] const CADETBLUE = 4284456608u32; #[doc =
    "-5247250"] const PALETURQUOISE = 4289720046u32; #[doc = "-2031617"] const LIGHTCYAN
    = 4292935679u32; #[doc = "-983041"] const AZURE = 4293984255u32; #[doc = "-14634326"]
    const LIGHTSEAGREEN = 4280332970u32; #[doc = "-12525360"] const TURQUOISE =
    4282441936u32; #[doc = "-5185306"] const POWDERBLUE = 4289781990u32; #[doc =
    "-13676721"] const DARKSLATEGRAY = 4281290575u32; #[doc = "-8388652"] const
    AQUAMARINE = 4286578644u32; #[doc = "-16713062"] const MEDIUMSPRINGGREEN =
    4278254234u32; #[doc = "-10039894"] const MEDIUMAQUAMARINE = 4284927402u32; #[doc =
    "-16711809"] const SPRINGGREEN = 4278255487u32; #[doc = "-12799119"] const
    MEDIUMSEAGREEN = 4282168177u32; #[doc = "-13726889"] const SEAGREEN = 4281240407u32;
    #[doc = "-13447886"] const LIMEGREEN = 4281519410u32; #[doc = "-16751616"] const
    DARKGREEN = 4278215680u32; #[doc = "-16744448"] const GREEN = 4278222848u32; #[doc =
    "-16711936"] const LIME = 4278255360u32; #[doc = "-14513374"] const FORESTGREEN =
    4280453922u32; #[doc = "-7357297"] const DARKSEAGREEN = 4287609999u32; #[doc =
    "-7278960"] const LIGHTGREEN = 4287688336u32; #[doc = "-6751336"] const PALEGREEN =
    4288215960u32; #[doc = "-655366"] const MINTCREAM = 4294311930u32; #[doc = "-983056"]
    const HONEYDEW = 4293984240u32; #[doc = "-8388864"] const CHARTREUSE = 4286578432u32;
    #[doc = "-8586240"] const LAWNGREEN = 4286381056u32; #[doc = "-9728477"] const
    OLIVEDRAB = 4285238819u32; #[doc = "-11179217"] const DARKOLIVEGREEN = 4283788079u32;
    #[doc = "-6632142"] const YELLOWGREEN = 4288335154u32; #[doc = "-5374161"] const
    GREENYELLOW = 4289593135u32; #[doc = "-657956"] const BEIGE = 4294309340u32; #[doc =
    "-331546"] const LINEN = 4294635750u32; #[doc = "-329006"] const LIGHTGOLDENRODYELLOW
    = 4294638290u32; #[doc = "-8355840"] const OLIVE = 4286611456u32; #[doc = "-256"]
    const YELLOW = 4294967040u32; #[doc = "-32"] const LIGHTYELLOW = 4294967264u32; #[doc
    = "-16"] const IVORY = 4294967280u32; #[doc = "-4343957"] const DARKKHAKI =
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
