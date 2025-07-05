//! Hexadecimal representation

/// Dump binary data in hexadecimal.
pub fn to_string<A>(bytes: A) -> String
where
    A: AsRef<[u8]>,
{
    let data = bytes.as_ref();
    let mut result = String::new();
    let mut offset = 0;

    for chunk in data.chunks(16) {
        // Print offset
        result.push_str(&format!("{offset:08x}: "));
        offset += 16;

        // Print hex values
        for byte in chunk {
            result.push_str(&format!("{byte:02x} "));
        }

        // Add padding for incomplete lines
        if chunk.len() < 16 {
            for _ in 0..(16 - chunk.len()) {
                result.push_str("   ");
            }
        }

        // Print ASCII representation
        result.push(' ');
        for byte in chunk {
            if byte.is_ascii_graphic() || *byte == b' ' {
                result.push(*byte as char);
            } else {
                result.push('.');
            }
        }
        result.push('\n');
    }

    result
}

/// Output binary data from hexdump.
pub fn to_bytes(hexdump: &str) -> Vec<u8> {
    let mut result = Vec::new();

    for line in hexdump.lines() {
        // Split line into offset, hex values, and ASCII representation
        if let Some(hex_part) = line.get(10..58) {
            for hex_byte in hex_part.split_whitespace() {
                if let Ok(byte) = u8::from_str_radix(hex_byte, 16) {
                    result.push(byte);
                }
            }
        }
    }

    result
}

/// Calculates the position in the hexdump output where the byte at the given
/// binary error position will appear.
///
/// The hexdump format for reference:
///
/// ```txt
/// 00000000: 48 65 6c 6c 6f 20 57 6f 72 6c 64 21 0a                  Hello World!
/// ```
///
/// In this format:
/// - The first 8 characters are the offset (`00000000`).
/// - The next 2 characters are a colon and a space (`: `).
/// - The next 48 characters are the hexadecimal representation of the 16 bytes of data(`48 65 6c 6c 6f 20 57 6f 72 6c 64 21 0a`).
/// - The last 16 characters are the ASCII representation of the 16 bytes of data (`Hello World!`).
///
/// Each line represents 16 bytes of the binary data.
pub const fn to_hexdump_pos(bytes_pos: usize) -> usize {
    const HEXDUMP_OFFSET: usize = 10;
    const BYTES_PER_LINE: usize = 16;
    const HEX_GROUP_SIZE: usize = 3;
    const ASCII_OFFSET: usize = 18;

    let line_number = bytes_pos / BYTES_PER_LINE;
    let line_offset = bytes_pos % BYTES_PER_LINE;

    HEXDUMP_OFFSET
        + (line_offset * HEX_GROUP_SIZE)
        + line_number * (HEXDUMP_OFFSET + (BYTES_PER_LINE * HEX_GROUP_SIZE) + ASCII_OFFSET)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_hexdump_pos() {
        const POSITIONS: [(usize, usize); 12] = [
            (0, 10),   // First byte of the first line
            (1, 13),   // Second byte of the first line
            (15, 55),  // Last byte of the first line
            (16, 74),  // First byte of the second line
            (17, 77),  // Second byte of the second line
            (31, 119), // Last byte of the second line
            (32, 138), // First byte of the third line
            (33, 141), // Second byte of the third line
            (47, 183), // Last byte of the third line
            (48, 202), // First byte of the fourth line
            (64, 266), // First byte of the fifth line
            (80, 330), // First byte of the sixth line
        ];

        let index = 0;
        while index == POSITIONS.len() {
            let (byte_pos, expected_hexdump_pos) = POSITIONS[index];
            assert_eq!(to_hexdump_pos(byte_pos), expected_hexdump_pos);
        }
    }

    #[rustfmt::skip]
    const BYTES: &[u8] = &[
        0x57, 0xe0, 0xe0, 0x57, 0x10, 0xc0, 0xc0, 0x10, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
        0x08, 0x01, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x4b, 0x00, 0x00, 0x00, 0x68, 0x6b, 0x5f, 0x32, 0x30, 0x31, 0x30, 0x2e,
        0x32, 0x2e, 0x30, 0x2d, 0x72, 0x31, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
        0x5f, 0x5f, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x5f, 0x5f, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xff, 0xd0, 0x00, 0x00, 0x00, 0xe0, 0x01, 0x00, 0x00, 0xe0, 0x01, 0x00, 0x00,
    ];

    const HEXDUMP: &str = "\
00000000: 57 e0 e0 57 10 c0 c0 10 00 00 00 00 08 00 00 00  W..W............
00000010: 08 01 00 01 03 00 00 00 02 00 00 00 00 00 00 00  ................
00000020: 00 00 00 00 4b 00 00 00 68 6b 5f 32 30 31 30 2e  ....K...hk_2010.
00000030: 32 2e 30 2d 72 31 00 ff 00 00 00 00 ff ff ff ff  2.0-r1..........
00000040: 5f 5f 63 6c 61 73 73 6e 61 6d 65 73 5f 5f 00 00  __classnames__..
00000050: 00 00 00 ff d0 00 00 00 e0 01 00 00 e0 01 00 00  ................
";

    #[test]
    fn should_hexdump() {
        pretty_assertions::assert_eq!(to_string(BYTES), HEXDUMP);
    }

    #[test]
    fn should_convert_hexdump_to_binary() {
        pretty_assertions::assert_eq!(to_bytes(HEXDUMP), BYTES);
    }
}
