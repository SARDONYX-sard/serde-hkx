// Copyright (c) 2006, NIF File Format Library and Tools All rights reserved. See niflib_LICENSE.txt

/// NIF file format version
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NifVersion {
    /// NIF Version 2.3
    Ver2_3 = 0x0203_0000,

    /// NIF Version 3.0
    Ver3_0 = 0x0300_0000,

    /// NIF Version 3.03
    Ver3_03 = 0x0300_0300,

    /// NIF Version 3.1
    Ver3_1 = 0x0301_0000,

    /// NIF Version 3.3.0.13
    Ver3_3_0_13 = 0x0303_000D,

    /// NIF Version 4.0.0.0
    Ver4_0_0_0 = 0x0400_0000,

    /// NIF Version 4.0.0.2
    Ver4_0_0_2 = 0x0400_0002,

    /// NIF Version 4.1.0.12
    Ver4_1_0_12 = 0x0401_000C,

    /// NIF Version 4.2.0.2
    Ver4_2_0_2 = 0x0402_0002,

    /// NIF Version 4.2.1.0
    Ver4_2_1_0 = 0x0402_0100,

    /// NIF Version 4.2.2.0
    Ver4_2_2_0 = 0x0402_0200,

    /// NIF Version 10.0.1.0
    Ver10_0_1_0 = 0x0A00_0100,

    /// NIF Version 10.0.1.2
    Ver10_0_1_2 = 0x0A00_0102,

    /// NIF Version 10.0.1.3
    Ver10_0_1_3 = 0x0A00_0103,

    /// NIF Version 10.1.0.0
    Ver10_1_0_0 = 0x0A01_0000,

    /// NIF Version 10.1.0.101
    Ver10_1_0_101 = 0x0A01_0065,

    /// NIF Version 10.1.0.106
    Ver10_1_0_106 = 0x0A01_006A,

    /// NIF Version 10.2.0.0
    Ver10_2_0_0 = 0x0A02_0000,

    /// NIF Version 10.4.0.1
    Ver10_4_0_1 = 0x0A04_0001,

    /// NIF Version 20.0.0.4
    Ver20_0_0_4 = 0x1400_0004,

    /// NIF Version 20.0.0.5
    Ver20_0_0_5 = 0x1400_0005,

    /// NIF Version 20.1.0.3
    Ver20_1_0_3 = 0x1401_0003,

    /// NIF Version 20.2.0.7
    #[default]
    Ver20_2_0_7 = 0x1402_0007,

    /// NIF Version 20.2.0.8
    Ver20_2_0_8 = 0x1402_0008,

    /// NIF Version 20.3.0.1
    Ver20_3_0_1 = 0x1403_0001,

    /// NIF Version 20.3.0.2
    Ver20_3_0_2 = 0x1403_0002,

    /// NIF Version 20.3.0.3
    Ver20_3_0_3 = 0x1403_0003,

    /// NIF Version 20.3.0.6
    Ver20_3_0_6 = 0x1403_0006,

    /// NIF Version 20.3.0.9
    Ver20_3_0_9 = 0x1403_0009,
}

#[cfg(feature = "clap")]
impl clap::ValueEnum for NifVersion {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Ver2_3,
            Self::Ver3_0,
            Self::Ver3_03,
            Self::Ver3_1,
            Self::Ver3_3_0_13,
            Self::Ver4_0_0_0,
            Self::Ver4_0_0_2,
            Self::Ver4_1_0_12,
            Self::Ver4_2_0_2,
            Self::Ver4_2_1_0,
            Self::Ver4_2_2_0,
            Self::Ver10_0_1_0,
            Self::Ver10_0_1_2,
            Self::Ver10_0_1_3,
            Self::Ver10_1_0_0,
            Self::Ver10_1_0_101,
            Self::Ver10_1_0_106,
            Self::Ver10_2_0_0,
            Self::Ver10_4_0_1,
            Self::Ver20_0_0_4,
            Self::Ver20_0_0_5,
            Self::Ver20_1_0_3,
            Self::Ver20_2_0_7,
            Self::Ver20_2_0_8,
            Self::Ver20_3_0_1,
            Self::Ver20_3_0_2,
            Self::Ver20_3_0_3,
            Self::Ver20_3_0_6,
            Self::Ver20_3_0_9,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        // Delegate to FromStr's string representation to keep a single source of truth.
        // Display must produce the same dot-separated string used in FromStr.
        Some(clap::builder::PossibleValue::new(match self {
            Self::Ver2_3 => "2.3",
            Self::Ver3_0 => "3.0",
            Self::Ver3_03 => "3.03",
            Self::Ver3_1 => "3.1",
            Self::Ver3_3_0_13 => "3.3.0.13",
            Self::Ver4_0_0_0 => "4.0.0.0",
            Self::Ver4_0_0_2 => "4.0.0.2",
            Self::Ver4_1_0_12 => "4.1.0.12",
            Self::Ver4_2_0_2 => "4.2.0.2",
            Self::Ver4_2_1_0 => "4.2.1.0",
            Self::Ver4_2_2_0 => "4.2.2.0",
            Self::Ver10_0_1_0 => "10.0.1.0",
            Self::Ver10_0_1_2 => "10.0.1.2",
            Self::Ver10_0_1_3 => "10.0.1.3",
            Self::Ver10_1_0_0 => "10.1.0.0",
            Self::Ver10_1_0_101 => "10.1.0.101",
            Self::Ver10_1_0_106 => "10.1.0.106",
            Self::Ver10_2_0_0 => "10.2.0.0",
            Self::Ver10_4_0_1 => "10.4.0.1",
            Self::Ver20_0_0_4 => "20.0.0.4",
            Self::Ver20_0_0_5 => "20.0.0.5",
            Self::Ver20_1_0_3 => "20.1.0.3",
            Self::Ver20_2_0_7 => "20.2.0.7",
            Self::Ver20_2_0_8 => "20.2.0.8",
            Self::Ver20_3_0_1 => "20.3.0.1",
            Self::Ver20_3_0_2 => "20.3.0.2",
            Self::Ver20_3_0_3 => "20.3.0.3",
            Self::Ver20_3_0_6 => "20.3.0.6",
            Self::Ver20_3_0_9 => "20.3.0.9",
        }))
    }
}
