use std::collections::HashMap;

use winnow::{
    binary::{self, Endianness},
    error::{ContextError, StrContext, StrContextValue::*},
    seq,
    token::take_while,
    Parser,
};

use crate::{bytes::de::parser::type_kind::string, tri};

/// - key: class name start offset
/// - value: class name
pub type ClassNames<'a> = HashMap<u32, &'a str>;

const FIXUP_VALUE_FOR_ALIGN: u32 = u32::MAX;

/// Create `local_fixups` from bytes.
///
/// # Returns
/// <class name start position, class name>
///
/// # Note
/// And take align mark(0xff) bytes.
pub fn classnames_section<'a>(
    endian: Endianness,
    base_offset: usize,
) -> impl Parser<&'a [u8], ClassNames<'a>, ContextError> {
    move |bytes: &mut &'a [u8]| {
        let mut class_map = HashMap::new();
        let mut offset = base_offset; // Necessary to determine the starting position of class_name.

        while let Ok(_signature) = binary::u32::<&[u8], ContextError>(endian)
            .verify(|src| *src != FIXUP_VALUE_FOR_ALIGN)
            .context(StrContext::Expected(Description("local_fixup.src(u32)")))
            .parse_next(bytes)
        {
            #[cfg(feature = "tracing")]
            tracing::trace!("signature: {_signature:#x}");

            let (class_name,) =tri!(seq! {
                _: binary::u8::<&[u8], ContextError>
                    .verify(|byte| *byte == 0x9)
                    .context(StrContext::Expected(Description("class name separator(0x9)"))),
                string // Parse until `\0`
                    .verify(|s: &str| s.is_ascii())
                    .context(StrContext::Expected(Description("ASCII class name(e.g. `hkReferencedObject`")))
            }.parse_next(bytes));

            offset += 5; // signature(4bytes) + separator(1byte)
            #[cfg(feature = "tracing")]
            tracing::trace!("class_name: {class_name}, offset: {offset}");
            class_map.insert(offset as u32, class_name);

            // Safety: as long as ASCII
            offset += class_name.len() + 1; // name + `\0` size
        }
        tri!(take_while(0.., 0xff).parse_next(bytes)); // take align mark bytes.
        Ok(class_map)
    }
}

#[cfg(test)]
mod tests {
    use super::classnames_section;
    use pretty_assertions::assert_eq;
    use winnow::{binary::Endianness, Parser};

    #[test]
    fn should_parse_classnames() {
        #[rustfmt::skip]
        let bytes= &[
         0xf6, 0x5e, 0x58, 0x75, 0x09, 0x68, 0x6b, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x00, 0xc2, 0xa4, 0x7e, // 000000d0:  .^Xu.hkClass...~
         0x5c, 0x09, 0x68, 0x6b, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x00, // 000000e0:  \.hkClassMember.
         0xcf, 0x09, 0x36, 0x8a, 0x09, 0x68, 0x6b, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x45, 0x6e, 0x75, 0x6d, // 000000f0:  ..6..hkClassEnum
         0x00, 0x6c, 0x8a, 0x6f, 0xce, 0x09, 0x68, 0x6b, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x45, 0x6e, 0x75, // 00000100:  .l.o..hkClassEnu
         0x6d, 0x49, 0x74, 0x65, 0x6d, 0x00, 0x1e, 0xc1, 0x72, 0x27, 0x09, 0x68, 0x6b, 0x52, 0x6f, 0x6f, // 00000110:  mItem...r'.hkRoo
         0x74, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x00, // 00000120:  tLevelContainer.
         0xa7, 0x9b, 0xa3, 0x13, 0x09, 0x68, 0x6b, 0x62, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x44, // 00000130:  .....hkbProjectD
         0x61, 0x74, 0x61, 0x00, 0x0a, 0xd6, 0x6a, 0x07, 0x09, 0x68, 0x6b, 0x62, 0x50, 0x72, 0x6f, 0x6a, // 00000140:  ata...j..hkbProj
         0x65, 0x63, 0x74, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x44, 0x61, 0x74, 0x61, 0x00, 0xff, 0xff, // 00000150:  ectStringData...
        ];

        match classnames_section(Endianness::Little, 0).parse(bytes) {
            Ok(class_map) => {
                let expected_class_map = [
                    (5, "hkClass"),
                    (18, "hkClassMember"),
                    (37, "hkClassEnum"),
                    (54, "hkClassEnumItem"),
                    (75, "hkRootLevelContainer"),
                    (101, "hkbProjectData"),
                    (121, "hkbProjectStringData"),
                ]
                .into();
                assert_eq!(class_map, expected_class_map);
            }
            Err(err) => panic!("{err}"),
        }
    }
}
