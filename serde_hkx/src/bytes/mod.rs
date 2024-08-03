//! Binary Serialization/Deserialization
pub mod de;
pub mod ser;
pub mod serde;

pub fn hexdump_string(data: &[u8]) -> String {
    let mut result = String::new();
    let mut offset = 0;

    for chunk in data.chunks(16) {
        // Print offset
        result.push_str(&format!("{:08x}: ", offset));
        offset += 16;

        // Print hex values
        for byte in chunk {
            result.push_str(&format!("{:02x} ", byte));
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

#[test]
fn should_hexdump() {
    let data = vec![
        0x57, 0xe0, 0xe0, 0x57, 0x10, 0xc0, 0xc0, 0x10, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00,
        0x00, 0x08, 0x01, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4b, 0x00, 0x00, 0x00, 0x68, 0x6b, 0x5f, 0x32, 0x30,
        0x31, 0x30, 0x2e, 0x32, 0x2e, 0x30, 0x2d, 0x72, 0x31, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xff, 0xff, 0xff, 0x5f, 0x5f, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x6e, 0x61, 0x6d, 0x65,
        0x73, 0x5f, 0x5f, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x00, 0x00, 0x00, 0xe0, 0x01,
        0x00, 0x00, 0xe0, 0x01, 0x00, 0x00,
    ];

    pretty_assertions::assert_eq!(
        hexdump_string(&data),
        r###"00000000: 57 e0 e0 57 10 c0 c0 10 00 00 00 00 08 00 00 00  W..W............
00000010: 08 01 00 01 03 00 00 00 02 00 00 00 00 00 00 00  ................
00000020: 00 00 00 00 4b 00 00 00 68 6b 5f 32 30 31 30 2e  ....K...hk_2010.
00000030: 32 2e 30 2d 72 31 00 ff 00 00 00 00 ff ff ff ff  2.0-r1..........
00000040: 5f 5f 63 6c 61 73 73 6e 61 6d 65 73 5f 5f 00 00  __classnames__..
00000050: 00 00 00 ff d0 00 00 00 e0 01 00 00 e0 01 00 00  ................
"###
    );
}
