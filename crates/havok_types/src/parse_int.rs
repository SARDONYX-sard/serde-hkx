//! Fast string parsing of radix(`0b`, `0o`, `0x`) using `lexical` crate

use core::num::NonZeroU8;
use lexical::NumberFormatBuilder;

const BASE_FORMAT: NumberFormatBuilder = NumberFormatBuilder::new()
    .integer_internal_digit_separator(true)
    .digit_separator(NonZeroU8::new(b'_'));

const BIN_FORMAT: u128 = BASE_FORMAT
    .radix(2)
    .base_prefix(NonZeroU8::new(b'b'))
    .base_prefix(NonZeroU8::new(b'B'))
    .build();
const OCTAL_FORMAT: u128 = BASE_FORMAT
    .radix(8)
    .base_prefix(NonZeroU8::new(b'o'))
    .base_prefix(NonZeroU8::new(b'O'))
    .build();
const HEX_FORMAT: u128 = BASE_FORMAT
    .radix(16)
    .base_prefix(NonZeroU8::new(b'x'))
    .base_prefix(NonZeroU8::new(b'X'))
    .build();

/// # Errors
#[inline]
fn parse_base<T>(input: &str, options: &T::Options) -> Result<T, lexical::Error>
where
    T: lexical::FromLexicalWithOptions + lexical::FromLexical,
{
    let prefix = if input.len() >= 2 { &input[0..2] } else { "" };
    let input = input.as_bytes();
    match prefix {
        "0b" | "0B" => T::from_lexical_with_options::<BIN_FORMAT>(input, options),
        "0o" | "0O" => T::from_lexical_with_options::<OCTAL_FORMAT>(input, options),
        "0x" | "0X" => T::from_lexical_with_options::<HEX_FORMAT>(input, options),
        _ => lexical::parse(input),
    }
}

// Trait implements
use lexical::parse_integer_options::Options;

/// A trait for parsing numeric values from strings.
///
/// Fast string parsing of radix(`0b`, `0o`, `0x`) with separator(`_`) using `lexical` crate.
pub trait ParseNumber: Sized {
    /// Parses a string into the target numeric type.
    ///
    /// # Errors
    /// Returns `lexical::Error` if parsing fails.
    ///
    /// # Examples
    /// ```
    /// use havok_types::parse_int::ParseNumber;
    /// let value: i64 = <i64 as ParseNumber>::parse("123").unwrap();
    /// assert_eq!(value, 123);
    /// ```
    fn parse(input: &str) -> Result<Self, lexical::Error>;

    /// Parses a string as `i64` or `u64` and casts it to the target type using `as`.
    ///
    /// # Behavior
    /// - If the target type is signed, it uses `i64` for parsing.
    /// - If the target type is unsigned, it uses `u64` for parsing.
    /// - The result is cast to the target type with `as`, allowing for wrapping behavior.
    ///
    /// # Errors
    /// Returns `lexical::Error` if parsing fails.
    ///
    /// # Examples
    /// ```
    /// use havok_types::parse_int::ParseNumber;
    /// let value: i8 = <i8 as ParseNumber>::parse_wrapping("300").unwrap();
    /// assert_eq!(value, 44);  // Wrapping overflow
    /// ```
    ///
    /// # Why do we need this?
    /// `hkFlag` is wrapped if a value greater than [`i16`] comes in, and is stringified to hexadecimal as [`u32`] at XML time.
    /// This method exists to reproduce that behavior.
    ///
    /// - Example: hkxcmd is setting the RoleFlags of `cow/behaviors/quadrupedbehavior.hkx` to `0xfffff300 (-3328)` in XML.
    ///   by wrapping the input hexadecimal number that is greater than `i16::MAX`
    /// ```
    /// use havok_types::parse_int::ParseNumber;
    /// const OVERFLOW_U32_STR: &str = "0xFFFFF300";
    /// assert_eq!(
    ///     <i16 as ParseNumber>::parse_wrapping(OVERFLOW_U32_STR).unwrap(),
    ///     -3328
    /// );
    /// assert_eq!(format!("{:#X}", (-3328_i16) as u32), OVERFLOW_U32_STR);
    /// ```
    fn parse_wrapping(input: &str) -> Result<Self, lexical::Error>;
}

/// Implements `ParseNumber` for all supported numeric types.
macro_rules! impl_parse_number {
    ($($t:ty),*) => {
        $(
            impl ParseNumber for $t {
                fn parse(input: &str) -> Result<Self, lexical::Error> {
                    parse_base(input, &Options::new())
                }

                fn parse_wrapping(input: &str) -> Result<Self, lexical::Error> {
                    if input.starts_with('-') {
                        // Parse as `i64` for signed types
                        let value = <i64 as ParseNumber>::parse(input)?;
                        Ok(value as Self)
                    } else {
                        // Parse as `u64` for unsigned types
                        let value = <u64 as ParseNumber>::parse(input)?;
                        Ok(value as Self)
                    }
                }
            }
        )*
    };
}

impl_parse_number!(
    i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers() {
        // 0xFFFFF300_u32 -3328_i16 (0xF300)
        let inputs = [
            ("0xFFFFF300", 4294963968_i64),   // Hexadecimal
            ("0b11000", 24),                  // Binary
            ("0B1111_1111_1111_1111", 65535), // Binary (separator + uppercase)
            ("0o11000", 4608),                // Octal
            ("0O71432", 29466),               // Octal (uppercase)
            ("01432", 1432),                  // Decimal (leading zero)
            ("3432", 3432),                   // Decimal
            ("0", 0),                         // Zero
        ];

        for (input, expected) in inputs {
            match <i64 as ParseNumber>::parse(input) {
                Ok(result) => assert_eq!(result, expected, "Failed to parse input: {}", input),
                Err(e) => panic!("Error parsing input {}: {}", input, e),
            }
        }
    }

    #[test]
    fn test_parse_wrapping_i16_u16() {
        // i16 and u16 cases

        // Havok specification
        {
            // - Example: hkxcmd is setting the RoleFlags of `cow/behaviors/quadrupedbehavior.hkx` to 0xfffff300 (-3328) in XML.
            //            by wrapping the input hexadecimal number that is greater than i16::MAX
            const OVERFLOW_U32_STR: &str = "0xFFFFF300";
            assert_eq!(
                <i16 as ParseNumber>::parse_wrapping(OVERFLOW_U32_STR).unwrap(),
                -3328
            );
            assert_eq!(format!("{:#X}", (-3328_i16) as u32), OVERFLOW_U32_STR);
        }

        assert_eq!(
            <u16 as ParseNumber>::parse_wrapping("0xFFFFF300").unwrap(),
            0xF300
        );

        assert_eq!(
            <i16 as ParseNumber>::parse_wrapping("0x7FFF").unwrap(),
            32767
        );
        assert_eq!(
            <u16 as ParseNumber>::parse_wrapping("0x7FFF").unwrap(),
            32767
        );

        assert_eq!(
            <i16 as ParseNumber>::parse_wrapping("0x8000").unwrap(),
            -32768
        );
        assert_eq!(
            <u16 as ParseNumber>::parse_wrapping("0x8000").unwrap(),
            32768
        );

        assert_eq!(<i16 as ParseNumber>::parse_wrapping("0xFFFF").unwrap(), -1);
        assert_eq!(
            <u16 as ParseNumber>::parse_wrapping("0xFFFF").unwrap(),
            65535
        );

        assert_eq!(<i16 as ParseNumber>::parse_wrapping("0x10000").unwrap(), 0);
        assert_eq!(<u16 as ParseNumber>::parse_wrapping("0x10000").unwrap(), 0);
    }

    #[test]
    fn test_parse_wrapping_i8_u8() {
        // i8 and u8 cases
        assert_eq!(
            <i8 as ParseNumber>::parse_wrapping("0b11111111").unwrap(),
            -1
        );
        assert_eq!(
            <u8 as ParseNumber>::parse_wrapping("0b11111111").unwrap(),
            255
        );

        assert_eq!(
            <i8 as ParseNumber>::parse_wrapping("0b10000000").unwrap(),
            -128
        );
        assert_eq!(
            <u8 as ParseNumber>::parse_wrapping("0b10000000").unwrap(),
            128
        );

        assert_eq!(<i8 as ParseNumber>::parse_wrapping("0xFF").unwrap(), -1);
        assert_eq!(<u8 as ParseNumber>::parse_wrapping("0xFF").unwrap(), 255);

        assert_eq!(<i8 as ParseNumber>::parse_wrapping("0x100").unwrap(), 0);
        assert_eq!(<u8 as ParseNumber>::parse_wrapping("0x100").unwrap(), 0);
    }

    #[test]
    fn test_parse_wrapping_overflow() {
        // Overflow behavior
        assert_eq!(<u8 as ParseNumber>::parse_wrapping("256").unwrap(), 0); // 256 % 256 = 0
        assert_eq!(<i8 as ParseNumber>::parse_wrapping("-129").unwrap(), 127); // -129 wraps to 127
    }
}
