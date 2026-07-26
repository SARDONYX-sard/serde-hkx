use std::{
    borrow::Cow,
    io::{self, ErrorKind},
};

use chardetng::{EncodingDetector, Iso2022JpDetection, Utf8Detection};

/// Converts an arbitrary byte slice to a UTF-8 `String` using automatic encoding detection.
///
/// Supported encodings include UTF-8, Shift_JIS, EUC-JP, ISO-2022-JP, etc.
/// Returns an `io::Error` if the input is empty or cannot be decoded properly.
///
/// # Errors
/// If input couldn't read.
pub fn decode_to_utf8(input: Vec<u8>) -> io::Result<String> {
    if input.is_empty() {
        return Ok(String::new());
    }

    // Detect encoding
    let mut detector = EncodingDetector::new(Iso2022JpDetection::Allow);
    detector.feed(&input, true);
    let encoding = detector.guess(Some(b"utf-8"), Utf8Detection::Allow);

    // Decode to UTF-8
    let (decoded, _, had_errors) = encoding.decode(&input);

    if had_errors {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "Failed to decode all characters",
        ));
    }

    Ok(match decoded {
        std::borrow::Cow::Borrowed(_s) => unsafe { String::from_utf8_unchecked(input) },
        std::borrow::Cow::Owned(s) => s,
    })
}

/// Converts an arbitrary byte slice to a UTF-8 `String` using automatic encoding detection.
///
/// Supported encodings include UTF-8, Shift_JIS, EUC-JP, ISO-2022-JP, etc.
/// Returns an `io::Error` if the input is empty or cannot be decoded properly.
///
/// # Errors
/// If input couldn't read.
pub fn decode_str_to_utf8(input: &[u8]) -> io::Result<Cow<'_, str>> {
    if input.is_empty() {
        return Ok(Cow::Borrowed(""));
    }
    let mut detector = EncodingDetector::new(Iso2022JpDetection::Allow);
    detector.feed(input, true);
    let encoding = detector.guess(Some(b"utf-8"), Utf8Detection::Allow);
    let (decoded, _, had_errors) = encoding.decode(input);
    if had_errors {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "Failed to decode all characters",
        ));
    }
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8() {
        let input = b"Hello, world!".to_vec();
        let result = decode_to_utf8(input).unwrap();
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_shift_jis() {
        // "こんにちは" in Shift_JIS
        let shift_jis_bytes = [0x82, 0xb1, 0x82, 0xf1, 0x82, 0xc9, 0x82, 0xbf, 0x82, 0xcd].to_vec();
        let result = decode_to_utf8(shift_jis_bytes).unwrap();
        assert_eq!(result, "こんにちは");
    }

    #[test]
    fn test_empty_input() {
        let result = decode_to_utf8(b"".to_vec()).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_invalid_bytes() {
        let bytes = [0xff, 0xfe, 0xfd].to_vec();
        let result = decode_to_utf8(bytes);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidData);
    }
}
