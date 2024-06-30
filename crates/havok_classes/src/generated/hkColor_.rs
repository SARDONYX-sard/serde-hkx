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
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkEnum<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_UINT32`"] #[doc = r""] #[doc =
    r" # Why this `enum` defined as `bitflags`?"] #[doc =
    r" These are not really `TYPE_FLAGS` but `TYPE_ENUM`, but since Rust does not allow the definition of"]
    #[doc =
    r" `enum` with duplicate discriminant values, they are defined as `bitflags`."]
    #[allow(non_upper_case_globals, non_snake_case)] #[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize))] #[repr(transparent)] #[derive(Debug,
    Clone, Copy, PartialEq, Eq, Hash)] pub struct ExtendedColors : u32 { #[doc =
    "4286578688"] const MAROON = 4286578688u32; #[doc = "4287299584"] const DARKRED =
    4287299584u32; #[doc = "4294901760"] const RED = 4294901760u32; #[doc = "4294944449"]
    const LIGHTPINK = 4294944449u32; #[doc = "4292613180"] const CRIMSON = 4292613180u32;
    #[doc = "4292789163"] const PALEVIOLETRED = 4292789163u32; #[doc = "4294927108"]
    const HOTPINK = 4294927108u32; #[doc = "4294907027"] const DEEPPINK = 4294907027u32;
    #[doc = "4291237253"] const MEDIUMVIOLETRED = 4291237253u32; #[doc = "4286578816"]
    const PURPLE = 4286578816u32; #[doc = "4287299723"] const DARKMAGENTA =
    4287299723u32; #[doc = "4292243670"] const ORCHID = 4292243670u32; #[doc =
    "4292394968"] const THISTLE = 4292394968u32; #[doc = "4292714717"] const PLUM =
    4292714717u32; #[doc = "4293821166"] const VIOLET = 4293821166u32; #[doc =
    "4294902015"] const FUCHSIA = 4294902015u32; #[doc = "4294902015"] const MAGENTA =
    4294902015u32; #[doc = "4290414451"] const MEDIUMORCHID = 4290414451u32; #[doc =
    "4287889619"] const DARKVIOLET = 4287889619u32; #[doc = "4288230092"] const
    DARKORCHID = 4288230092u32; #[doc = "4287245282"] const BLUEVIOLET = 4287245282u32;
    #[doc = "4284875570"] const INDIGO = 4284875570u32; #[doc = "4289454203"] const
    MEDIUMPURPLE = 4289454203u32; #[doc = "4287317261"] const SLATEBLUE = 4287317261u32;
    #[doc = "4288269460"] const MEDIUMSLATEBLUE = 4288269460u32; #[doc = "4287299587"]
    const DARKBLUE = 4287299587u32; #[doc = "4289379277"] const MEDIUMBLUE =
    4289379277u32; #[doc = "4294901760"] const BLUE = 4294901760u32; #[doc =
    "4284875648"] const NAVY = 4284875648u32; #[doc = "4281368880"] const MIDNIGHTBLUE =
    4281368880u32; #[doc = "4284048699"] const DARKSLATEBLUE = 4284048699u32; #[doc =
    "4286006955"] const ROYALBLUE = 4286006955u32; #[doc = "4284782061"] const
    CORNFLOWERBLUE = 4284782061u32; #[doc = "4289775366"] const LIGHTSTEELBLUE =
    4289775366u32; #[doc = "4293982463"] const ALICEBLUE = 4293982463u32; #[doc =
    "4294638335"] const GHOSTWHITE = 4294638335u32; #[doc = "4293322490"] const LAVENDER
    = 4293322490u32; #[doc = "4280193279"] const DODGERBLUE = 4280193279u32; #[doc =
    "4282811060"] const STEELBLUE = 4282811060u32; #[doc = "4278255615"] const
    DEEPSKYBLUE = 4278255615u32; #[doc = "4285835392"] const SLATEGRAY = 4285835392u32;
    #[doc = "4286023833"] const LIGHTSLATEGRAY = 4286023833u32; #[doc = "4289339194"]
    const LIGHTSKYBLUE = 4289339194u32; #[doc = "4289342075"] const SKYBLUE =
    4289342075u32; #[doc = "4289959174"] const LIGHTBLUE = 4289959174u32; #[doc =
    "4284829576"] const TEAL = 4284829576u32; #[doc = "4287358955"] const DARKCYAN =
    4287358955u32; #[doc = "4289470265"] const DARKTURQUOISE = 4289470265u32; #[doc =
    "4286582015"] const CYAN = 4286582015u32; #[doc = "4289789324"] const MEDIUMTURQUOISE
    = 4289789324u32; #[doc = "4288893830"] const CADETBLUE = 4288893830u32; #[doc =
    "4289520374"] const PALETURQUOISE = 4289520374u32; #[doc = "4292935679"] const
    LIGHTCYAN = 4292935679u32; #[doc = "4293984255"] const AZURE = 4293984255u32; #[doc =
    "4280332970"] const LIGHTSEAGREEN = 4280332970u32; #[doc = "4283796272"] const
    TURQUOISE = 4283796272u32; #[doc = "4289774812"] const POWDERBLUE = 4289774812u32;
    #[doc = "4281522975"] const DARKSLATEGRAY = 4281522975u32; #[doc = "4289379196"]
    const AQUAMARINE = 4289379196u32; #[doc = "4278258842"] const MEDIUMSPRINGGREEN =
    4278258842u32; #[doc = "4288723946"] const MEDIUMAQUAMARINE = 4288723946u32; #[doc =
    "4286563067"] const SPRINGGREEN = 4286563067u32; #[doc = "4285864305"] const
    MEDIUMSEAGREEN = 4285864305u32; #[doc = "4284861671"] const SEAGREEN = 4284861671u32;
    #[doc = "4288347394"] const LIMEGREEN = 4288347394u32; #[doc = "4284827999"] const
    DARKGREEN = 4284827999u32; #[doc = "4284829695"] const GREEN = 4284829695u32; #[doc =
    "4286563010"] const LIME = 4286563010u32; #[doc = "4286526978"] const FORESTGREEN =
    4286526978u32; #[doc = "4292029967"] const DARKSEAGREEN = 4292029967u32; #[doc =
    "4292148896"] const LIGHTGREEN = 4292148896u32; #[doc = "4292430344"] const PALEGREEN
    = 4292430344u32; #[doc = "4294966780"] const MINTCREAM = 4294966780u32; #[doc =
    "4293987744"] const HONEYDEW = 4293987744u32; #[doc = "4289370336"] const CHARTREUSE
    = 4289370336u32; #[doc = "4289336832"] const LAWNGREEN = 4289336832u32; #[doc =
    "4286085249"] const OLIVEDRAB = 4286085249u32; #[doc = "4283676211"] const
    DARKOLIVEGREEN = 4283676211u32; #[doc = "4292272306"] const YELLOWGREEN =
    4292272306u32; #[doc = "4293960297"] const GREENYELLOW = 4293960297u32; #[doc =
    "4293787924"] const BEIGE = 4293787924u32; #[doc = "4294635702"] const LINEN =
    4294635702u32; #[doc = "4294637750"] const LIGHTGOLDENRODYELLOW = 4294637750u32;
    #[doc = "4286595072"] const OLIVE = 4286595072u32; #[doc = "4294967040"] const YELLOW
    = 4294967040u32; #[doc = "4294967216"] const LIGHTYELLOW = 4294967216u32; #[doc =
    "4294967280"] const IVORY = 4294967280u32; #[doc = "4290623339"] const DARKKHAKI =
    4290623339u32; #[doc = "4293988284"] const KHAKI = 4293988284u32; #[doc =
    "4293467754"] const PALEGOLDENROD = 4293467754u32; #[doc = "4293720580"] const WHEAT
    = 4293720580u32; #[doc = "4294956800"] const GOLD = 4294956800u32; #[doc =
    "4294965453"] const LEMONCHIFFON = 4294965453u32; #[doc = "4294955917"] const
    PAPAYAWHIP = 4294955917u32; #[doc = "4286903435"] const DARKGOLDENROD =
    4286903435u32; #[doc = "4292519200"] const GOLDENROD = 4292519200u32; #[doc =
    "4294634455"] const ANTIQUEWHITE = 4294634455u32; #[doc = "4294966116"] const
    CORNSILK = 4294966116u32; #[doc = "4294960102"] const OLDLACE = 4294960102u32; #[doc
    = "4294953679"] const MOCCASIN = 4294953679u32; #[doc = "4294956353"] const
    NAVAJOWHITE = 4294956353u32; #[doc = "4294944000"] const ORANGE = 4294944000u32;
    #[doc = "4294953924"] const BISQUE = 4294953924u32; #[doc = "4292907392"] const TAN =
    4292907392u32; #[doc = "4294940672"] const DARKORANGE = 4294940672u32; #[doc =
    "4292783287"] const BURLYWOOD = 4292783287u32; #[doc = "4287317267"] const
    SADDLEBROWN = 4287317267u32; #[doc = "4294096452"] const SANDYBROWN = 4294096452u32;
    #[doc = "4294962125"] const BLANCHEDALMOND = 4294962125u32; #[doc = "4294964715"]
    const LAVENDERBLUSH = 4294964715u32; #[doc = "4294965998"] const SEASHELL =
    4294965998u32; #[doc = "4294966496"] const FLORALWHITE = 4294966496u32; #[doc =
    "4294967182"] const SNOW = 4294967182u32; #[doc = "4286078079"] const PERU =
    4286078079u32; #[doc = "4294956489"] const PEACHPUFF = 4294956489u32; #[doc =
    "4289864222"] const CHOCOLATE = 4289864222u32; #[doc = "4288696877"] const SIENNA =
    4288696877u32; #[doc = "4294942842"] const LIGHTSALMON = 4294942842u32; #[doc =
    "4294934352"] const CORAL = 4294934352u32; #[doc = "4293997764"] const DARKSALMON =
    4293997764u32; #[doc = "4294954237"] const MISTYROSE = 4294954237u32; #[doc =
    "4294940672"] const ORANGERED = 4294940672u32; #[doc = "4294947974"] const SALMON =
    4294947974u32; #[doc = "4294943895"] const TOMATO = 4294943895u32; #[doc =
    "4292080495"] const ROSYBROWN = 4292080495u32; #[doc = "4294961403"] const PINK =
    4294961403u32; #[doc = "4289864222"] const INDIANRED = 4289864222u32; #[doc =
    "4293959497"] const LIGHTCORAL = 4293959497u32; #[doc = "4289014314"] const BROWN =
    4289014314u32; #[doc = "4289501062"] const FIREBRICK = 4289501062u32; #[doc =
    "4278190080"] const BLACK = 4278190080u32; #[doc = "4285098345"] const DIMGRAY =
    4285098345u32; #[doc = "4286611584"] const GRAY = 4286611584u32; #[doc =
    "4289309097"] const DARKGRAY = 4289309097u32; #[doc = "4289374890"] const SILVER =
    4289374890u32; #[doc = "4292072403"] const LIGHTGREY = 4292072403u32; #[doc =
    "4292332748"] const GAINSBORO = 4292332748u32; #[doc = "4294506751"] const WHITESMOKE
    = 4294506751u32; #[doc = "4294967295"] const WHITE = 4294967295u32; #[doc =
    "4287137928"] const GREY = 4287137928u32; #[doc = "4282400896"] const GREY25 =
    4282400896u32; #[doc = "4286611584"] const GREY50 = 4286611584u32; #[doc =
    "4290822336"] const GREY75 = 4290822336u32; }
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
                Self::MAROON => __serializer.serialize_field("MAROON", &4286578688u64),
                Self::DARKRED => __serializer.serialize_field("DARKRED", &4287299584u64),
                Self::RED => __serializer.serialize_field("RED", &4294901760u64),
                Self::LIGHTPINK => {
                    __serializer.serialize_field("LIGHTPINK", &4294944449u64)
                }
                Self::CRIMSON => __serializer.serialize_field("CRIMSON", &4292613180u64),
                Self::PALEVIOLETRED => {
                    __serializer.serialize_field("PALEVIOLETRED", &4292789163u64)
                }
                Self::HOTPINK => __serializer.serialize_field("HOTPINK", &4294927108u64),
                Self::DEEPPINK => {
                    __serializer.serialize_field("DEEPPINK", &4294907027u64)
                }
                Self::MEDIUMVIOLETRED => {
                    __serializer.serialize_field("MEDIUMVIOLETRED", &4291237253u64)
                }
                Self::PURPLE => __serializer.serialize_field("PURPLE", &4286578816u64),
                Self::DARKMAGENTA => {
                    __serializer.serialize_field("DARKMAGENTA", &4287299723u64)
                }
                Self::ORCHID => __serializer.serialize_field("ORCHID", &4292243670u64),
                Self::THISTLE => __serializer.serialize_field("THISTLE", &4292394968u64),
                Self::PLUM => __serializer.serialize_field("PLUM", &4292714717u64),
                Self::VIOLET => __serializer.serialize_field("VIOLET", &4293821166u64),
                Self::FUCHSIA => __serializer.serialize_field("FUCHSIA", &4294902015u64),
                Self::MAGENTA => __serializer.serialize_field("MAGENTA", &4294902015u64),
                Self::MEDIUMORCHID => {
                    __serializer.serialize_field("MEDIUMORCHID", &4290414451u64)
                }
                Self::DARKVIOLET => {
                    __serializer.serialize_field("DARKVIOLET", &4287889619u64)
                }
                Self::DARKORCHID => {
                    __serializer.serialize_field("DARKORCHID", &4288230092u64)
                }
                Self::BLUEVIOLET => {
                    __serializer.serialize_field("BLUEVIOLET", &4287245282u64)
                }
                Self::INDIGO => __serializer.serialize_field("INDIGO", &4284875570u64),
                Self::MEDIUMPURPLE => {
                    __serializer.serialize_field("MEDIUMPURPLE", &4289454203u64)
                }
                Self::SLATEBLUE => {
                    __serializer.serialize_field("SLATEBLUE", &4287317261u64)
                }
                Self::MEDIUMSLATEBLUE => {
                    __serializer.serialize_field("MEDIUMSLATEBLUE", &4288269460u64)
                }
                Self::DARKBLUE => {
                    __serializer.serialize_field("DARKBLUE", &4287299587u64)
                }
                Self::MEDIUMBLUE => {
                    __serializer.serialize_field("MEDIUMBLUE", &4289379277u64)
                }
                Self::BLUE => __serializer.serialize_field("BLUE", &4294901760u64),
                Self::NAVY => __serializer.serialize_field("NAVY", &4284875648u64),
                Self::MIDNIGHTBLUE => {
                    __serializer.serialize_field("MIDNIGHTBLUE", &4281368880u64)
                }
                Self::DARKSLATEBLUE => {
                    __serializer.serialize_field("DARKSLATEBLUE", &4284048699u64)
                }
                Self::ROYALBLUE => {
                    __serializer.serialize_field("ROYALBLUE", &4286006955u64)
                }
                Self::CORNFLOWERBLUE => {
                    __serializer.serialize_field("CORNFLOWERBLUE", &4284782061u64)
                }
                Self::LIGHTSTEELBLUE => {
                    __serializer.serialize_field("LIGHTSTEELBLUE", &4289775366u64)
                }
                Self::ALICEBLUE => {
                    __serializer.serialize_field("ALICEBLUE", &4293982463u64)
                }
                Self::GHOSTWHITE => {
                    __serializer.serialize_field("GHOSTWHITE", &4294638335u64)
                }
                Self::LAVENDER => {
                    __serializer.serialize_field("LAVENDER", &4293322490u64)
                }
                Self::DODGERBLUE => {
                    __serializer.serialize_field("DODGERBLUE", &4280193279u64)
                }
                Self::STEELBLUE => {
                    __serializer.serialize_field("STEELBLUE", &4282811060u64)
                }
                Self::DEEPSKYBLUE => {
                    __serializer.serialize_field("DEEPSKYBLUE", &4278255615u64)
                }
                Self::SLATEGRAY => {
                    __serializer.serialize_field("SLATEGRAY", &4285835392u64)
                }
                Self::LIGHTSLATEGRAY => {
                    __serializer.serialize_field("LIGHTSLATEGRAY", &4286023833u64)
                }
                Self::LIGHTSKYBLUE => {
                    __serializer.serialize_field("LIGHTSKYBLUE", &4289339194u64)
                }
                Self::SKYBLUE => __serializer.serialize_field("SKYBLUE", &4289342075u64),
                Self::LIGHTBLUE => {
                    __serializer.serialize_field("LIGHTBLUE", &4289959174u64)
                }
                Self::TEAL => __serializer.serialize_field("TEAL", &4284829576u64),
                Self::DARKCYAN => {
                    __serializer.serialize_field("DARKCYAN", &4287358955u64)
                }
                Self::DARKTURQUOISE => {
                    __serializer.serialize_field("DARKTURQUOISE", &4289470265u64)
                }
                Self::CYAN => __serializer.serialize_field("CYAN", &4286582015u64),
                Self::MEDIUMTURQUOISE => {
                    __serializer.serialize_field("MEDIUMTURQUOISE", &4289789324u64)
                }
                Self::CADETBLUE => {
                    __serializer.serialize_field("CADETBLUE", &4288893830u64)
                }
                Self::PALETURQUOISE => {
                    __serializer.serialize_field("PALETURQUOISE", &4289520374u64)
                }
                Self::LIGHTCYAN => {
                    __serializer.serialize_field("LIGHTCYAN", &4292935679u64)
                }
                Self::AZURE => __serializer.serialize_field("AZURE", &4293984255u64),
                Self::LIGHTSEAGREEN => {
                    __serializer.serialize_field("LIGHTSEAGREEN", &4280332970u64)
                }
                Self::TURQUOISE => {
                    __serializer.serialize_field("TURQUOISE", &4283796272u64)
                }
                Self::POWDERBLUE => {
                    __serializer.serialize_field("POWDERBLUE", &4289774812u64)
                }
                Self::DARKSLATEGRAY => {
                    __serializer.serialize_field("DARKSLATEGRAY", &4281522975u64)
                }
                Self::AQUAMARINE => {
                    __serializer.serialize_field("AQUAMARINE", &4289379196u64)
                }
                Self::MEDIUMSPRINGGREEN => {
                    __serializer.serialize_field("MEDIUMSPRINGGREEN", &4278258842u64)
                }
                Self::MEDIUMAQUAMARINE => {
                    __serializer.serialize_field("MEDIUMAQUAMARINE", &4288723946u64)
                }
                Self::SPRINGGREEN => {
                    __serializer.serialize_field("SPRINGGREEN", &4286563067u64)
                }
                Self::MEDIUMSEAGREEN => {
                    __serializer.serialize_field("MEDIUMSEAGREEN", &4285864305u64)
                }
                Self::SEAGREEN => {
                    __serializer.serialize_field("SEAGREEN", &4284861671u64)
                }
                Self::LIMEGREEN => {
                    __serializer.serialize_field("LIMEGREEN", &4288347394u64)
                }
                Self::DARKGREEN => {
                    __serializer.serialize_field("DARKGREEN", &4284827999u64)
                }
                Self::GREEN => __serializer.serialize_field("GREEN", &4284829695u64),
                Self::LIME => __serializer.serialize_field("LIME", &4286563010u64),
                Self::FORESTGREEN => {
                    __serializer.serialize_field("FORESTGREEN", &4286526978u64)
                }
                Self::DARKSEAGREEN => {
                    __serializer.serialize_field("DARKSEAGREEN", &4292029967u64)
                }
                Self::LIGHTGREEN => {
                    __serializer.serialize_field("LIGHTGREEN", &4292148896u64)
                }
                Self::PALEGREEN => {
                    __serializer.serialize_field("PALEGREEN", &4292430344u64)
                }
                Self::MINTCREAM => {
                    __serializer.serialize_field("MINTCREAM", &4294966780u64)
                }
                Self::HONEYDEW => {
                    __serializer.serialize_field("HONEYDEW", &4293987744u64)
                }
                Self::CHARTREUSE => {
                    __serializer.serialize_field("CHARTREUSE", &4289370336u64)
                }
                Self::LAWNGREEN => {
                    __serializer.serialize_field("LAWNGREEN", &4289336832u64)
                }
                Self::OLIVEDRAB => {
                    __serializer.serialize_field("OLIVEDRAB", &4286085249u64)
                }
                Self::DARKOLIVEGREEN => {
                    __serializer.serialize_field("DARKOLIVEGREEN", &4283676211u64)
                }
                Self::YELLOWGREEN => {
                    __serializer.serialize_field("YELLOWGREEN", &4292272306u64)
                }
                Self::GREENYELLOW => {
                    __serializer.serialize_field("GREENYELLOW", &4293960297u64)
                }
                Self::BEIGE => __serializer.serialize_field("BEIGE", &4293787924u64),
                Self::LINEN => __serializer.serialize_field("LINEN", &4294635702u64),
                Self::LIGHTGOLDENRODYELLOW => {
                    __serializer.serialize_field("LIGHTGOLDENRODYELLOW", &4294637750u64)
                }
                Self::OLIVE => __serializer.serialize_field("OLIVE", &4286595072u64),
                Self::YELLOW => __serializer.serialize_field("YELLOW", &4294967040u64),
                Self::LIGHTYELLOW => {
                    __serializer.serialize_field("LIGHTYELLOW", &4294967216u64)
                }
                Self::IVORY => __serializer.serialize_field("IVORY", &4294967280u64),
                Self::DARKKHAKI => {
                    __serializer.serialize_field("DARKKHAKI", &4290623339u64)
                }
                Self::KHAKI => __serializer.serialize_field("KHAKI", &4293988284u64),
                Self::PALEGOLDENROD => {
                    __serializer.serialize_field("PALEGOLDENROD", &4293467754u64)
                }
                Self::WHEAT => __serializer.serialize_field("WHEAT", &4293720580u64),
                Self::GOLD => __serializer.serialize_field("GOLD", &4294956800u64),
                Self::LEMONCHIFFON => {
                    __serializer.serialize_field("LEMONCHIFFON", &4294965453u64)
                }
                Self::PAPAYAWHIP => {
                    __serializer.serialize_field("PAPAYAWHIP", &4294955917u64)
                }
                Self::DARKGOLDENROD => {
                    __serializer.serialize_field("DARKGOLDENROD", &4286903435u64)
                }
                Self::GOLDENROD => {
                    __serializer.serialize_field("GOLDENROD", &4292519200u64)
                }
                Self::ANTIQUEWHITE => {
                    __serializer.serialize_field("ANTIQUEWHITE", &4294634455u64)
                }
                Self::CORNSILK => {
                    __serializer.serialize_field("CORNSILK", &4294966116u64)
                }
                Self::OLDLACE => __serializer.serialize_field("OLDLACE", &4294960102u64),
                Self::MOCCASIN => {
                    __serializer.serialize_field("MOCCASIN", &4294953679u64)
                }
                Self::NAVAJOWHITE => {
                    __serializer.serialize_field("NAVAJOWHITE", &4294956353u64)
                }
                Self::ORANGE => __serializer.serialize_field("ORANGE", &4294944000u64),
                Self::BISQUE => __serializer.serialize_field("BISQUE", &4294953924u64),
                Self::TAN => __serializer.serialize_field("TAN", &4292907392u64),
                Self::DARKORANGE => {
                    __serializer.serialize_field("DARKORANGE", &4294940672u64)
                }
                Self::BURLYWOOD => {
                    __serializer.serialize_field("BURLYWOOD", &4292783287u64)
                }
                Self::SADDLEBROWN => {
                    __serializer.serialize_field("SADDLEBROWN", &4287317267u64)
                }
                Self::SANDYBROWN => {
                    __serializer.serialize_field("SANDYBROWN", &4294096452u64)
                }
                Self::BLANCHEDALMOND => {
                    __serializer.serialize_field("BLANCHEDALMOND", &4294962125u64)
                }
                Self::LAVENDERBLUSH => {
                    __serializer.serialize_field("LAVENDERBLUSH", &4294964715u64)
                }
                Self::SEASHELL => {
                    __serializer.serialize_field("SEASHELL", &4294965998u64)
                }
                Self::FLORALWHITE => {
                    __serializer.serialize_field("FLORALWHITE", &4294966496u64)
                }
                Self::SNOW => __serializer.serialize_field("SNOW", &4294967182u64),
                Self::PERU => __serializer.serialize_field("PERU", &4286078079u64),
                Self::PEACHPUFF => {
                    __serializer.serialize_field("PEACHPUFF", &4294956489u64)
                }
                Self::CHOCOLATE => {
                    __serializer.serialize_field("CHOCOLATE", &4289864222u64)
                }
                Self::SIENNA => __serializer.serialize_field("SIENNA", &4288696877u64),
                Self::LIGHTSALMON => {
                    __serializer.serialize_field("LIGHTSALMON", &4294942842u64)
                }
                Self::CORAL => __serializer.serialize_field("CORAL", &4294934352u64),
                Self::DARKSALMON => {
                    __serializer.serialize_field("DARKSALMON", &4293997764u64)
                }
                Self::MISTYROSE => {
                    __serializer.serialize_field("MISTYROSE", &4294954237u64)
                }
                Self::ORANGERED => {
                    __serializer.serialize_field("ORANGERED", &4294940672u64)
                }
                Self::SALMON => __serializer.serialize_field("SALMON", &4294947974u64),
                Self::TOMATO => __serializer.serialize_field("TOMATO", &4294943895u64),
                Self::ROSYBROWN => {
                    __serializer.serialize_field("ROSYBROWN", &4292080495u64)
                }
                Self::PINK => __serializer.serialize_field("PINK", &4294961403u64),
                Self::INDIANRED => {
                    __serializer.serialize_field("INDIANRED", &4289864222u64)
                }
                Self::LIGHTCORAL => {
                    __serializer.serialize_field("LIGHTCORAL", &4293959497u64)
                }
                Self::BROWN => __serializer.serialize_field("BROWN", &4289014314u64),
                Self::FIREBRICK => {
                    __serializer.serialize_field("FIREBRICK", &4289501062u64)
                }
                Self::BLACK => __serializer.serialize_field("BLACK", &4278190080u64),
                Self::DIMGRAY => __serializer.serialize_field("DIMGRAY", &4285098345u64),
                Self::GRAY => __serializer.serialize_field("GRAY", &4286611584u64),
                Self::DARKGRAY => {
                    __serializer.serialize_field("DARKGRAY", &4289309097u64)
                }
                Self::SILVER => __serializer.serialize_field("SILVER", &4289374890u64),
                Self::LIGHTGREY => {
                    __serializer.serialize_field("LIGHTGREY", &4292072403u64)
                }
                Self::GAINSBORO => {
                    __serializer.serialize_field("GAINSBORO", &4292332748u64)
                }
                Self::WHITESMOKE => {
                    __serializer.serialize_field("WHITESMOKE", &4294506751u64)
                }
                Self::WHITE => __serializer.serialize_field("WHITE", &4294967295u64),
                Self::GREY => __serializer.serialize_field("GREY", &4287137928u64),
                Self::GREY25 => __serializer.serialize_field("GREY25", &4282400896u64),
                Self::GREY50 => __serializer.serialize_field("GREY50", &4286611584u64),
                Self::GREY75 => __serializer.serialize_field("GREY75", &4290822336u64),
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
                    __value: u32,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        4286578688u32 => _serde::__private::Ok(__Field::__field0),
                        4287299584u32 => _serde::__private::Ok(__Field::__field1),
                        4294901760u32 => _serde::__private::Ok(__Field::__field2),
                        4294944449u32 => _serde::__private::Ok(__Field::__field3),
                        4292613180u32 => _serde::__private::Ok(__Field::__field4),
                        4292789163u32 => _serde::__private::Ok(__Field::__field5),
                        4294927108u32 => _serde::__private::Ok(__Field::__field6),
                        4294907027u32 => _serde::__private::Ok(__Field::__field7),
                        4291237253u32 => _serde::__private::Ok(__Field::__field8),
                        4286578816u32 => _serde::__private::Ok(__Field::__field9),
                        4287299723u32 => _serde::__private::Ok(__Field::__field10),
                        4292243670u32 => _serde::__private::Ok(__Field::__field11),
                        4292394968u32 => _serde::__private::Ok(__Field::__field12),
                        4292714717u32 => _serde::__private::Ok(__Field::__field13),
                        4293821166u32 => _serde::__private::Ok(__Field::__field14),
                        4294902015u32 => _serde::__private::Ok(__Field::__field15),
                        4294902015u32 => _serde::__private::Ok(__Field::__field16),
                        4290414451u32 => _serde::__private::Ok(__Field::__field17),
                        4287889619u32 => _serde::__private::Ok(__Field::__field18),
                        4288230092u32 => _serde::__private::Ok(__Field::__field19),
                        4287245282u32 => _serde::__private::Ok(__Field::__field20),
                        4284875570u32 => _serde::__private::Ok(__Field::__field21),
                        4289454203u32 => _serde::__private::Ok(__Field::__field22),
                        4287317261u32 => _serde::__private::Ok(__Field::__field23),
                        4288269460u32 => _serde::__private::Ok(__Field::__field24),
                        4287299587u32 => _serde::__private::Ok(__Field::__field25),
                        4289379277u32 => _serde::__private::Ok(__Field::__field26),
                        4294901760u32 => _serde::__private::Ok(__Field::__field27),
                        4284875648u32 => _serde::__private::Ok(__Field::__field28),
                        4281368880u32 => _serde::__private::Ok(__Field::__field29),
                        4284048699u32 => _serde::__private::Ok(__Field::__field30),
                        4286006955u32 => _serde::__private::Ok(__Field::__field31),
                        4284782061u32 => _serde::__private::Ok(__Field::__field32),
                        4289775366u32 => _serde::__private::Ok(__Field::__field33),
                        4293982463u32 => _serde::__private::Ok(__Field::__field34),
                        4294638335u32 => _serde::__private::Ok(__Field::__field35),
                        4293322490u32 => _serde::__private::Ok(__Field::__field36),
                        4280193279u32 => _serde::__private::Ok(__Field::__field37),
                        4282811060u32 => _serde::__private::Ok(__Field::__field38),
                        4278255615u32 => _serde::__private::Ok(__Field::__field39),
                        4285835392u32 => _serde::__private::Ok(__Field::__field40),
                        4286023833u32 => _serde::__private::Ok(__Field::__field41),
                        4289339194u32 => _serde::__private::Ok(__Field::__field42),
                        4289342075u32 => _serde::__private::Ok(__Field::__field43),
                        4289959174u32 => _serde::__private::Ok(__Field::__field44),
                        4284829576u32 => _serde::__private::Ok(__Field::__field45),
                        4287358955u32 => _serde::__private::Ok(__Field::__field46),
                        4289470265u32 => _serde::__private::Ok(__Field::__field47),
                        4286582015u32 => _serde::__private::Ok(__Field::__field48),
                        4289789324u32 => _serde::__private::Ok(__Field::__field49),
                        4288893830u32 => _serde::__private::Ok(__Field::__field50),
                        4289520374u32 => _serde::__private::Ok(__Field::__field51),
                        4292935679u32 => _serde::__private::Ok(__Field::__field52),
                        4293984255u32 => _serde::__private::Ok(__Field::__field53),
                        4280332970u32 => _serde::__private::Ok(__Field::__field54),
                        4283796272u32 => _serde::__private::Ok(__Field::__field55),
                        4289774812u32 => _serde::__private::Ok(__Field::__field56),
                        4281522975u32 => _serde::__private::Ok(__Field::__field57),
                        4289379196u32 => _serde::__private::Ok(__Field::__field58),
                        4278258842u32 => _serde::__private::Ok(__Field::__field59),
                        4288723946u32 => _serde::__private::Ok(__Field::__field60),
                        4286563067u32 => _serde::__private::Ok(__Field::__field61),
                        4285864305u32 => _serde::__private::Ok(__Field::__field62),
                        4284861671u32 => _serde::__private::Ok(__Field::__field63),
                        4288347394u32 => _serde::__private::Ok(__Field::__field64),
                        4284827999u32 => _serde::__private::Ok(__Field::__field65),
                        4284829695u32 => _serde::__private::Ok(__Field::__field66),
                        4286563010u32 => _serde::__private::Ok(__Field::__field67),
                        4286526978u32 => _serde::__private::Ok(__Field::__field68),
                        4292029967u32 => _serde::__private::Ok(__Field::__field69),
                        4292148896u32 => _serde::__private::Ok(__Field::__field70),
                        4292430344u32 => _serde::__private::Ok(__Field::__field71),
                        4294966780u32 => _serde::__private::Ok(__Field::__field72),
                        4293987744u32 => _serde::__private::Ok(__Field::__field73),
                        4289370336u32 => _serde::__private::Ok(__Field::__field74),
                        4289336832u32 => _serde::__private::Ok(__Field::__field75),
                        4286085249u32 => _serde::__private::Ok(__Field::__field76),
                        4283676211u32 => _serde::__private::Ok(__Field::__field77),
                        4292272306u32 => _serde::__private::Ok(__Field::__field78),
                        4293960297u32 => _serde::__private::Ok(__Field::__field79),
                        4293787924u32 => _serde::__private::Ok(__Field::__field80),
                        4294635702u32 => _serde::__private::Ok(__Field::__field81),
                        4294637750u32 => _serde::__private::Ok(__Field::__field82),
                        4286595072u32 => _serde::__private::Ok(__Field::__field83),
                        4294967040u32 => _serde::__private::Ok(__Field::__field84),
                        4294967216u32 => _serde::__private::Ok(__Field::__field85),
                        4294967280u32 => _serde::__private::Ok(__Field::__field86),
                        4290623339u32 => _serde::__private::Ok(__Field::__field87),
                        4293988284u32 => _serde::__private::Ok(__Field::__field88),
                        4293467754u32 => _serde::__private::Ok(__Field::__field89),
                        4293720580u32 => _serde::__private::Ok(__Field::__field90),
                        4294956800u32 => _serde::__private::Ok(__Field::__field91),
                        4294965453u32 => _serde::__private::Ok(__Field::__field92),
                        4294955917u32 => _serde::__private::Ok(__Field::__field93),
                        4286903435u32 => _serde::__private::Ok(__Field::__field94),
                        4292519200u32 => _serde::__private::Ok(__Field::__field95),
                        4294634455u32 => _serde::__private::Ok(__Field::__field96),
                        4294966116u32 => _serde::__private::Ok(__Field::__field97),
                        4294960102u32 => _serde::__private::Ok(__Field::__field98),
                        4294953679u32 => _serde::__private::Ok(__Field::__field99),
                        4294956353u32 => _serde::__private::Ok(__Field::__field100),
                        4294944000u32 => _serde::__private::Ok(__Field::__field101),
                        4294953924u32 => _serde::__private::Ok(__Field::__field102),
                        4292907392u32 => _serde::__private::Ok(__Field::__field103),
                        4294940672u32 => _serde::__private::Ok(__Field::__field104),
                        4292783287u32 => _serde::__private::Ok(__Field::__field105),
                        4287317267u32 => _serde::__private::Ok(__Field::__field106),
                        4294096452u32 => _serde::__private::Ok(__Field::__field107),
                        4294962125u32 => _serde::__private::Ok(__Field::__field108),
                        4294964715u32 => _serde::__private::Ok(__Field::__field109),
                        4294965998u32 => _serde::__private::Ok(__Field::__field110),
                        4294966496u32 => _serde::__private::Ok(__Field::__field111),
                        4294967182u32 => _serde::__private::Ok(__Field::__field112),
                        4286078079u32 => _serde::__private::Ok(__Field::__field113),
                        4294956489u32 => _serde::__private::Ok(__Field::__field114),
                        4289864222u32 => _serde::__private::Ok(__Field::__field115),
                        4288696877u32 => _serde::__private::Ok(__Field::__field116),
                        4294942842u32 => _serde::__private::Ok(__Field::__field117),
                        4294934352u32 => _serde::__private::Ok(__Field::__field118),
                        4293997764u32 => _serde::__private::Ok(__Field::__field119),
                        4294954237u32 => _serde::__private::Ok(__Field::__field120),
                        4294940672u32 => _serde::__private::Ok(__Field::__field121),
                        4294947974u32 => _serde::__private::Ok(__Field::__field122),
                        4294943895u32 => _serde::__private::Ok(__Field::__field123),
                        4292080495u32 => _serde::__private::Ok(__Field::__field124),
                        4294961403u32 => _serde::__private::Ok(__Field::__field125),
                        4289864222u32 => _serde::__private::Ok(__Field::__field126),
                        4293959497u32 => _serde::__private::Ok(__Field::__field127),
                        4289014314u32 => _serde::__private::Ok(__Field::__field128),
                        4289501062u32 => _serde::__private::Ok(__Field::__field129),
                        4278190080u32 => _serde::__private::Ok(__Field::__field130),
                        4285098345u32 => _serde::__private::Ok(__Field::__field131),
                        4286611584u32 => _serde::__private::Ok(__Field::__field132),
                        4289309097u32 => _serde::__private::Ok(__Field::__field133),
                        4289374890u32 => _serde::__private::Ok(__Field::__field134),
                        4292072403u32 => _serde::__private::Ok(__Field::__field135),
                        4292332748u32 => _serde::__private::Ok(__Field::__field136),
                        4294506751u32 => _serde::__private::Ok(__Field::__field137),
                        4294967295u32 => _serde::__private::Ok(__Field::__field138),
                        4287137928u32 => _serde::__private::Ok(__Field::__field139),
                        4282400896u32 => _serde::__private::Ok(__Field::__field140),
                        4286611584u32 => _serde::__private::Ok(__Field::__field141),
                        4290822336u32 => _serde::__private::Ok(__Field::__field142),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint32(__value),
                                    &"value(u32) of variant is one of 4286578688, 4287299584, 4294901760, 4294944449, 4292613180, 4292789163, 4294927108, 4294907027, 4291237253, 4286578816, 4287299723, 4292243670, 4292394968, 4292714717, 4293821166, 4294902015, 4294902015, 4290414451, 4287889619, 4288230092, 4287245282, 4284875570, 4289454203, 4287317261, 4288269460, 4287299587, 4289379277, 4294901760, 4284875648, 4281368880, 4284048699, 4286006955, 4284782061, 4289775366, 4293982463, 4294638335, 4293322490, 4280193279, 4282811060, 4278255615, 4285835392, 4286023833, 4289339194, 4289342075, 4289959174, 4284829576, 4287358955, 4289470265, 4286582015, 4289789324, 4288893830, 4289520374, 4292935679, 4293984255, 4280332970, 4283796272, 4289774812, 4281522975, 4289379196, 4278258842, 4288723946, 4286563067, 4285864305, 4284861671, 4288347394, 4284827999, 4284829695, 4286563010, 4286526978, 4292029967, 4292148896, 4292430344, 4294966780, 4293987744, 4289370336, 4289336832, 4286085249, 4283676211, 4292272306, 4293960297, 4293787924, 4294635702, 4294637750, 4286595072, 4294967040, 4294967216, 4294967280, 4290623339, 4293988284, 4293467754, 4293720580, 4294956800, 4294965453, 4294955917, 4286903435, 4292519200, 4294634455, 4294966116, 4294960102, 4294953679, 4294956353, 4294944000, 4294953924, 4292907392, 4294940672, 4292783287, 4287317267, 4294096452, 4294962125, 4294964715, 4294965998, 4294966496, 4294967182, 4286078079, 4294956489, 4289864222, 4288696877, 4294942842, 4294934352, 4293997764, 4294954237, 4294940672, 4294947974, 4294943895, 4292080495, 4294961403, 4289864222, 4293959497, 4289014314, 4289501062, 4278190080, 4285098345, 4286611584, 4289309097, 4289374890, 4292072403, 4292332748, 4294506751, 4294967295, 4287137928, 4282400896, 4286611584, 4290822336",
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
                            v if v == "4286578688"
                                || v.eq_ignore_ascii_case("MAROON") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "4287299584"
                                || v.eq_ignore_ascii_case("DARKRED") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "4294901760" || v.eq_ignore_ascii_case("RED") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "4294944449"
                                || v.eq_ignore_ascii_case("LIGHTPINK") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4292613180"
                                || v.eq_ignore_ascii_case("CRIMSON") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "4292789163"
                                || v.eq_ignore_ascii_case("PALEVIOLETRED") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "4294927108"
                                || v.eq_ignore_ascii_case("HOTPINK") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "4294907027"
                                || v.eq_ignore_ascii_case("DEEPPINK") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "4291237253"
                                || v.eq_ignore_ascii_case("MEDIUMVIOLETRED") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "4286578816"
                                || v.eq_ignore_ascii_case("PURPLE") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "4287299723"
                                || v.eq_ignore_ascii_case("DARKMAGENTA") => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            v if v == "4292243670"
                                || v.eq_ignore_ascii_case("ORCHID") => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            v if v == "4292394968"
                                || v.eq_ignore_ascii_case("THISTLE") => {
                                _serde::__private::Ok(__Field::__field12)
                            }
                            v if v == "4292714717" || v.eq_ignore_ascii_case("PLUM") => {
                                _serde::__private::Ok(__Field::__field13)
                            }
                            v if v == "4293821166"
                                || v.eq_ignore_ascii_case("VIOLET") => {
                                _serde::__private::Ok(__Field::__field14)
                            }
                            v if v == "4294902015"
                                || v.eq_ignore_ascii_case("FUCHSIA") => {
                                _serde::__private::Ok(__Field::__field15)
                            }
                            v if v == "4294902015"
                                || v.eq_ignore_ascii_case("MAGENTA") => {
                                _serde::__private::Ok(__Field::__field16)
                            }
                            v if v == "4290414451"
                                || v.eq_ignore_ascii_case("MEDIUMORCHID") => {
                                _serde::__private::Ok(__Field::__field17)
                            }
                            v if v == "4287889619"
                                || v.eq_ignore_ascii_case("DARKVIOLET") => {
                                _serde::__private::Ok(__Field::__field18)
                            }
                            v if v == "4288230092"
                                || v.eq_ignore_ascii_case("DARKORCHID") => {
                                _serde::__private::Ok(__Field::__field19)
                            }
                            v if v == "4287245282"
                                || v.eq_ignore_ascii_case("BLUEVIOLET") => {
                                _serde::__private::Ok(__Field::__field20)
                            }
                            v if v == "4284875570"
                                || v.eq_ignore_ascii_case("INDIGO") => {
                                _serde::__private::Ok(__Field::__field21)
                            }
                            v if v == "4289454203"
                                || v.eq_ignore_ascii_case("MEDIUMPURPLE") => {
                                _serde::__private::Ok(__Field::__field22)
                            }
                            v if v == "4287317261"
                                || v.eq_ignore_ascii_case("SLATEBLUE") => {
                                _serde::__private::Ok(__Field::__field23)
                            }
                            v if v == "4288269460"
                                || v.eq_ignore_ascii_case("MEDIUMSLATEBLUE") => {
                                _serde::__private::Ok(__Field::__field24)
                            }
                            v if v == "4287299587"
                                || v.eq_ignore_ascii_case("DARKBLUE") => {
                                _serde::__private::Ok(__Field::__field25)
                            }
                            v if v == "4289379277"
                                || v.eq_ignore_ascii_case("MEDIUMBLUE") => {
                                _serde::__private::Ok(__Field::__field26)
                            }
                            v if v == "4294901760" || v.eq_ignore_ascii_case("BLUE") => {
                                _serde::__private::Ok(__Field::__field27)
                            }
                            v if v == "4284875648" || v.eq_ignore_ascii_case("NAVY") => {
                                _serde::__private::Ok(__Field::__field28)
                            }
                            v if v == "4281368880"
                                || v.eq_ignore_ascii_case("MIDNIGHTBLUE") => {
                                _serde::__private::Ok(__Field::__field29)
                            }
                            v if v == "4284048699"
                                || v.eq_ignore_ascii_case("DARKSLATEBLUE") => {
                                _serde::__private::Ok(__Field::__field30)
                            }
                            v if v == "4286006955"
                                || v.eq_ignore_ascii_case("ROYALBLUE") => {
                                _serde::__private::Ok(__Field::__field31)
                            }
                            v if v == "4284782061"
                                || v.eq_ignore_ascii_case("CORNFLOWERBLUE") => {
                                _serde::__private::Ok(__Field::__field32)
                            }
                            v if v == "4289775366"
                                || v.eq_ignore_ascii_case("LIGHTSTEELBLUE") => {
                                _serde::__private::Ok(__Field::__field33)
                            }
                            v if v == "4293982463"
                                || v.eq_ignore_ascii_case("ALICEBLUE") => {
                                _serde::__private::Ok(__Field::__field34)
                            }
                            v if v == "4294638335"
                                || v.eq_ignore_ascii_case("GHOSTWHITE") => {
                                _serde::__private::Ok(__Field::__field35)
                            }
                            v if v == "4293322490"
                                || v.eq_ignore_ascii_case("LAVENDER") => {
                                _serde::__private::Ok(__Field::__field36)
                            }
                            v if v == "4280193279"
                                || v.eq_ignore_ascii_case("DODGERBLUE") => {
                                _serde::__private::Ok(__Field::__field37)
                            }
                            v if v == "4282811060"
                                || v.eq_ignore_ascii_case("STEELBLUE") => {
                                _serde::__private::Ok(__Field::__field38)
                            }
                            v if v == "4278255615"
                                || v.eq_ignore_ascii_case("DEEPSKYBLUE") => {
                                _serde::__private::Ok(__Field::__field39)
                            }
                            v if v == "4285835392"
                                || v.eq_ignore_ascii_case("SLATEGRAY") => {
                                _serde::__private::Ok(__Field::__field40)
                            }
                            v if v == "4286023833"
                                || v.eq_ignore_ascii_case("LIGHTSLATEGRAY") => {
                                _serde::__private::Ok(__Field::__field41)
                            }
                            v if v == "4289339194"
                                || v.eq_ignore_ascii_case("LIGHTSKYBLUE") => {
                                _serde::__private::Ok(__Field::__field42)
                            }
                            v if v == "4289342075"
                                || v.eq_ignore_ascii_case("SKYBLUE") => {
                                _serde::__private::Ok(__Field::__field43)
                            }
                            v if v == "4289959174"
                                || v.eq_ignore_ascii_case("LIGHTBLUE") => {
                                _serde::__private::Ok(__Field::__field44)
                            }
                            v if v == "4284829576" || v.eq_ignore_ascii_case("TEAL") => {
                                _serde::__private::Ok(__Field::__field45)
                            }
                            v if v == "4287358955"
                                || v.eq_ignore_ascii_case("DARKCYAN") => {
                                _serde::__private::Ok(__Field::__field46)
                            }
                            v if v == "4289470265"
                                || v.eq_ignore_ascii_case("DARKTURQUOISE") => {
                                _serde::__private::Ok(__Field::__field47)
                            }
                            v if v == "4286582015" || v.eq_ignore_ascii_case("CYAN") => {
                                _serde::__private::Ok(__Field::__field48)
                            }
                            v if v == "4289789324"
                                || v.eq_ignore_ascii_case("MEDIUMTURQUOISE") => {
                                _serde::__private::Ok(__Field::__field49)
                            }
                            v if v == "4288893830"
                                || v.eq_ignore_ascii_case("CADETBLUE") => {
                                _serde::__private::Ok(__Field::__field50)
                            }
                            v if v == "4289520374"
                                || v.eq_ignore_ascii_case("PALETURQUOISE") => {
                                _serde::__private::Ok(__Field::__field51)
                            }
                            v if v == "4292935679"
                                || v.eq_ignore_ascii_case("LIGHTCYAN") => {
                                _serde::__private::Ok(__Field::__field52)
                            }
                            v if v == "4293984255" || v.eq_ignore_ascii_case("AZURE") => {
                                _serde::__private::Ok(__Field::__field53)
                            }
                            v if v == "4280332970"
                                || v.eq_ignore_ascii_case("LIGHTSEAGREEN") => {
                                _serde::__private::Ok(__Field::__field54)
                            }
                            v if v == "4283796272"
                                || v.eq_ignore_ascii_case("TURQUOISE") => {
                                _serde::__private::Ok(__Field::__field55)
                            }
                            v if v == "4289774812"
                                || v.eq_ignore_ascii_case("POWDERBLUE") => {
                                _serde::__private::Ok(__Field::__field56)
                            }
                            v if v == "4281522975"
                                || v.eq_ignore_ascii_case("DARKSLATEGRAY") => {
                                _serde::__private::Ok(__Field::__field57)
                            }
                            v if v == "4289379196"
                                || v.eq_ignore_ascii_case("AQUAMARINE") => {
                                _serde::__private::Ok(__Field::__field58)
                            }
                            v if v == "4278258842"
                                || v.eq_ignore_ascii_case("MEDIUMSPRINGGREEN") => {
                                _serde::__private::Ok(__Field::__field59)
                            }
                            v if v == "4288723946"
                                || v.eq_ignore_ascii_case("MEDIUMAQUAMARINE") => {
                                _serde::__private::Ok(__Field::__field60)
                            }
                            v if v == "4286563067"
                                || v.eq_ignore_ascii_case("SPRINGGREEN") => {
                                _serde::__private::Ok(__Field::__field61)
                            }
                            v if v == "4285864305"
                                || v.eq_ignore_ascii_case("MEDIUMSEAGREEN") => {
                                _serde::__private::Ok(__Field::__field62)
                            }
                            v if v == "4284861671"
                                || v.eq_ignore_ascii_case("SEAGREEN") => {
                                _serde::__private::Ok(__Field::__field63)
                            }
                            v if v == "4288347394"
                                || v.eq_ignore_ascii_case("LIMEGREEN") => {
                                _serde::__private::Ok(__Field::__field64)
                            }
                            v if v == "4284827999"
                                || v.eq_ignore_ascii_case("DARKGREEN") => {
                                _serde::__private::Ok(__Field::__field65)
                            }
                            v if v == "4284829695" || v.eq_ignore_ascii_case("GREEN") => {
                                _serde::__private::Ok(__Field::__field66)
                            }
                            v if v == "4286563010" || v.eq_ignore_ascii_case("LIME") => {
                                _serde::__private::Ok(__Field::__field67)
                            }
                            v if v == "4286526978"
                                || v.eq_ignore_ascii_case("FORESTGREEN") => {
                                _serde::__private::Ok(__Field::__field68)
                            }
                            v if v == "4292029967"
                                || v.eq_ignore_ascii_case("DARKSEAGREEN") => {
                                _serde::__private::Ok(__Field::__field69)
                            }
                            v if v == "4292148896"
                                || v.eq_ignore_ascii_case("LIGHTGREEN") => {
                                _serde::__private::Ok(__Field::__field70)
                            }
                            v if v == "4292430344"
                                || v.eq_ignore_ascii_case("PALEGREEN") => {
                                _serde::__private::Ok(__Field::__field71)
                            }
                            v if v == "4294966780"
                                || v.eq_ignore_ascii_case("MINTCREAM") => {
                                _serde::__private::Ok(__Field::__field72)
                            }
                            v if v == "4293987744"
                                || v.eq_ignore_ascii_case("HONEYDEW") => {
                                _serde::__private::Ok(__Field::__field73)
                            }
                            v if v == "4289370336"
                                || v.eq_ignore_ascii_case("CHARTREUSE") => {
                                _serde::__private::Ok(__Field::__field74)
                            }
                            v if v == "4289336832"
                                || v.eq_ignore_ascii_case("LAWNGREEN") => {
                                _serde::__private::Ok(__Field::__field75)
                            }
                            v if v == "4286085249"
                                || v.eq_ignore_ascii_case("OLIVEDRAB") => {
                                _serde::__private::Ok(__Field::__field76)
                            }
                            v if v == "4283676211"
                                || v.eq_ignore_ascii_case("DARKOLIVEGREEN") => {
                                _serde::__private::Ok(__Field::__field77)
                            }
                            v if v == "4292272306"
                                || v.eq_ignore_ascii_case("YELLOWGREEN") => {
                                _serde::__private::Ok(__Field::__field78)
                            }
                            v if v == "4293960297"
                                || v.eq_ignore_ascii_case("GREENYELLOW") => {
                                _serde::__private::Ok(__Field::__field79)
                            }
                            v if v == "4293787924" || v.eq_ignore_ascii_case("BEIGE") => {
                                _serde::__private::Ok(__Field::__field80)
                            }
                            v if v == "4294635702" || v.eq_ignore_ascii_case("LINEN") => {
                                _serde::__private::Ok(__Field::__field81)
                            }
                            v if v == "4294637750"
                                || v.eq_ignore_ascii_case("LIGHTGOLDENRODYELLOW") => {
                                _serde::__private::Ok(__Field::__field82)
                            }
                            v if v == "4286595072" || v.eq_ignore_ascii_case("OLIVE") => {
                                _serde::__private::Ok(__Field::__field83)
                            }
                            v if v == "4294967040"
                                || v.eq_ignore_ascii_case("YELLOW") => {
                                _serde::__private::Ok(__Field::__field84)
                            }
                            v if v == "4294967216"
                                || v.eq_ignore_ascii_case("LIGHTYELLOW") => {
                                _serde::__private::Ok(__Field::__field85)
                            }
                            v if v == "4294967280" || v.eq_ignore_ascii_case("IVORY") => {
                                _serde::__private::Ok(__Field::__field86)
                            }
                            v if v == "4290623339"
                                || v.eq_ignore_ascii_case("DARKKHAKI") => {
                                _serde::__private::Ok(__Field::__field87)
                            }
                            v if v == "4293988284" || v.eq_ignore_ascii_case("KHAKI") => {
                                _serde::__private::Ok(__Field::__field88)
                            }
                            v if v == "4293467754"
                                || v.eq_ignore_ascii_case("PALEGOLDENROD") => {
                                _serde::__private::Ok(__Field::__field89)
                            }
                            v if v == "4293720580" || v.eq_ignore_ascii_case("WHEAT") => {
                                _serde::__private::Ok(__Field::__field90)
                            }
                            v if v == "4294956800" || v.eq_ignore_ascii_case("GOLD") => {
                                _serde::__private::Ok(__Field::__field91)
                            }
                            v if v == "4294965453"
                                || v.eq_ignore_ascii_case("LEMONCHIFFON") => {
                                _serde::__private::Ok(__Field::__field92)
                            }
                            v if v == "4294955917"
                                || v.eq_ignore_ascii_case("PAPAYAWHIP") => {
                                _serde::__private::Ok(__Field::__field93)
                            }
                            v if v == "4286903435"
                                || v.eq_ignore_ascii_case("DARKGOLDENROD") => {
                                _serde::__private::Ok(__Field::__field94)
                            }
                            v if v == "4292519200"
                                || v.eq_ignore_ascii_case("GOLDENROD") => {
                                _serde::__private::Ok(__Field::__field95)
                            }
                            v if v == "4294634455"
                                || v.eq_ignore_ascii_case("ANTIQUEWHITE") => {
                                _serde::__private::Ok(__Field::__field96)
                            }
                            v if v == "4294966116"
                                || v.eq_ignore_ascii_case("CORNSILK") => {
                                _serde::__private::Ok(__Field::__field97)
                            }
                            v if v == "4294960102"
                                || v.eq_ignore_ascii_case("OLDLACE") => {
                                _serde::__private::Ok(__Field::__field98)
                            }
                            v if v == "4294953679"
                                || v.eq_ignore_ascii_case("MOCCASIN") => {
                                _serde::__private::Ok(__Field::__field99)
                            }
                            v if v == "4294956353"
                                || v.eq_ignore_ascii_case("NAVAJOWHITE") => {
                                _serde::__private::Ok(__Field::__field100)
                            }
                            v if v == "4294944000"
                                || v.eq_ignore_ascii_case("ORANGE") => {
                                _serde::__private::Ok(__Field::__field101)
                            }
                            v if v == "4294953924"
                                || v.eq_ignore_ascii_case("BISQUE") => {
                                _serde::__private::Ok(__Field::__field102)
                            }
                            v if v == "4292907392" || v.eq_ignore_ascii_case("TAN") => {
                                _serde::__private::Ok(__Field::__field103)
                            }
                            v if v == "4294940672"
                                || v.eq_ignore_ascii_case("DARKORANGE") => {
                                _serde::__private::Ok(__Field::__field104)
                            }
                            v if v == "4292783287"
                                || v.eq_ignore_ascii_case("BURLYWOOD") => {
                                _serde::__private::Ok(__Field::__field105)
                            }
                            v if v == "4287317267"
                                || v.eq_ignore_ascii_case("SADDLEBROWN") => {
                                _serde::__private::Ok(__Field::__field106)
                            }
                            v if v == "4294096452"
                                || v.eq_ignore_ascii_case("SANDYBROWN") => {
                                _serde::__private::Ok(__Field::__field107)
                            }
                            v if v == "4294962125"
                                || v.eq_ignore_ascii_case("BLANCHEDALMOND") => {
                                _serde::__private::Ok(__Field::__field108)
                            }
                            v if v == "4294964715"
                                || v.eq_ignore_ascii_case("LAVENDERBLUSH") => {
                                _serde::__private::Ok(__Field::__field109)
                            }
                            v if v == "4294965998"
                                || v.eq_ignore_ascii_case("SEASHELL") => {
                                _serde::__private::Ok(__Field::__field110)
                            }
                            v if v == "4294966496"
                                || v.eq_ignore_ascii_case("FLORALWHITE") => {
                                _serde::__private::Ok(__Field::__field111)
                            }
                            v if v == "4294967182" || v.eq_ignore_ascii_case("SNOW") => {
                                _serde::__private::Ok(__Field::__field112)
                            }
                            v if v == "4286078079" || v.eq_ignore_ascii_case("PERU") => {
                                _serde::__private::Ok(__Field::__field113)
                            }
                            v if v == "4294956489"
                                || v.eq_ignore_ascii_case("PEACHPUFF") => {
                                _serde::__private::Ok(__Field::__field114)
                            }
                            v if v == "4289864222"
                                || v.eq_ignore_ascii_case("CHOCOLATE") => {
                                _serde::__private::Ok(__Field::__field115)
                            }
                            v if v == "4288696877"
                                || v.eq_ignore_ascii_case("SIENNA") => {
                                _serde::__private::Ok(__Field::__field116)
                            }
                            v if v == "4294942842"
                                || v.eq_ignore_ascii_case("LIGHTSALMON") => {
                                _serde::__private::Ok(__Field::__field117)
                            }
                            v if v == "4294934352" || v.eq_ignore_ascii_case("CORAL") => {
                                _serde::__private::Ok(__Field::__field118)
                            }
                            v if v == "4293997764"
                                || v.eq_ignore_ascii_case("DARKSALMON") => {
                                _serde::__private::Ok(__Field::__field119)
                            }
                            v if v == "4294954237"
                                || v.eq_ignore_ascii_case("MISTYROSE") => {
                                _serde::__private::Ok(__Field::__field120)
                            }
                            v if v == "4294940672"
                                || v.eq_ignore_ascii_case("ORANGERED") => {
                                _serde::__private::Ok(__Field::__field121)
                            }
                            v if v == "4294947974"
                                || v.eq_ignore_ascii_case("SALMON") => {
                                _serde::__private::Ok(__Field::__field122)
                            }
                            v if v == "4294943895"
                                || v.eq_ignore_ascii_case("TOMATO") => {
                                _serde::__private::Ok(__Field::__field123)
                            }
                            v if v == "4292080495"
                                || v.eq_ignore_ascii_case("ROSYBROWN") => {
                                _serde::__private::Ok(__Field::__field124)
                            }
                            v if v == "4294961403" || v.eq_ignore_ascii_case("PINK") => {
                                _serde::__private::Ok(__Field::__field125)
                            }
                            v if v == "4289864222"
                                || v.eq_ignore_ascii_case("INDIANRED") => {
                                _serde::__private::Ok(__Field::__field126)
                            }
                            v if v == "4293959497"
                                || v.eq_ignore_ascii_case("LIGHTCORAL") => {
                                _serde::__private::Ok(__Field::__field127)
                            }
                            v if v == "4289014314" || v.eq_ignore_ascii_case("BROWN") => {
                                _serde::__private::Ok(__Field::__field128)
                            }
                            v if v == "4289501062"
                                || v.eq_ignore_ascii_case("FIREBRICK") => {
                                _serde::__private::Ok(__Field::__field129)
                            }
                            v if v == "4278190080" || v.eq_ignore_ascii_case("BLACK") => {
                                _serde::__private::Ok(__Field::__field130)
                            }
                            v if v == "4285098345"
                                || v.eq_ignore_ascii_case("DIMGRAY") => {
                                _serde::__private::Ok(__Field::__field131)
                            }
                            v if v == "4286611584" || v.eq_ignore_ascii_case("GRAY") => {
                                _serde::__private::Ok(__Field::__field132)
                            }
                            v if v == "4289309097"
                                || v.eq_ignore_ascii_case("DARKGRAY") => {
                                _serde::__private::Ok(__Field::__field133)
                            }
                            v if v == "4289374890"
                                || v.eq_ignore_ascii_case("SILVER") => {
                                _serde::__private::Ok(__Field::__field134)
                            }
                            v if v == "4292072403"
                                || v.eq_ignore_ascii_case("LIGHTGREY") => {
                                _serde::__private::Ok(__Field::__field135)
                            }
                            v if v == "4292332748"
                                || v.eq_ignore_ascii_case("GAINSBORO") => {
                                _serde::__private::Ok(__Field::__field136)
                            }
                            v if v == "4294506751"
                                || v.eq_ignore_ascii_case("WHITESMOKE") => {
                                _serde::__private::Ok(__Field::__field137)
                            }
                            v if v == "4294967295" || v.eq_ignore_ascii_case("WHITE") => {
                                _serde::__private::Ok(__Field::__field138)
                            }
                            v if v == "4287137928" || v.eq_ignore_ascii_case("GREY") => {
                                _serde::__private::Ok(__Field::__field139)
                            }
                            v if v == "4282400896"
                                || v.eq_ignore_ascii_case("GREY25") => {
                                _serde::__private::Ok(__Field::__field140)
                            }
                            v if v == "4286611584"
                                || v.eq_ignore_ascii_case("GREY50") => {
                                _serde::__private::Ok(__Field::__field141)
                            }
                            v if v == "4290822336"
                                || v.eq_ignore_ascii_case("GREY75") => {
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
                _serde::de::ReadEnumSize::Uint32,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ExtendedColors>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
