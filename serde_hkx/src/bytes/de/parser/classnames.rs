use std::collections::HashMap;

use winnow::{
    binary::{self, Endianness},
    error::{ContextError, StrContext, StrContextValue::*},
    token::take_while,
    Parser,
};

use crate::{bytes::de::parser::type_kind::string, tri};

/// - key: class name start offset
/// - value: class name
pub type ClassNames<'a> = HashMap<u32, &'a str>;

const FIXUP_VALUE_FOR_ALIGN: u32 = 0xffffff;

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
        let mut offset = base_offset;

        while let Ok(signature) = binary::u32::<&[u8], ContextError>(endian)
            .verify(|src| *src != FIXUP_VALUE_FOR_ALIGN)
            .context(StrContext::Expected(Description("local_fixup.src(u32)")))
            .parse_next(bytes)
        {
            #[cfg(feature = "tracing")]
            tracing::trace!("{signature:#x}");
            offset += 4;

            tri!(binary::u8::<&[u8], ContextError>
                .verify(|byte| *byte == 0x9)
                .context(StrContext::Expected(Description(
                    "class name separator(0x9)"
                )))
                .parse_next(bytes));
            offset += 1;

            let class_name = tri!(string()
                .verify(|s: &str| s.chars().all(|c| c.is_ascii()))
                .context(StrContext::Expected(Description(
                    "ASCII class name(e.g. `hkReferencedObject`"
                )))
                .parse_next(bytes));
            offset += class_name.len(); // Safe as long as ASCII;

            class_map.insert(offset as u32, class_name);
        }
        tri!(take_while(0.., 0xff).parse_next(bytes)); // take align mark bytes.
        Ok(class_map)
    }
}
