use super::mock_requires::*;

bitflags::bitflags! {
    /// Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++).
    ///
    /// # On XML
    /// When all bits are 0, "0" is inserted.
    /// (Even if `FLAGS_NONE = 0` and 0 is replaced by `FLAGS_NONE`, "0" will appear when reconverting xml -> hkx -> xml.)
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct FlagValues: u16 {
        /// Flags is empty: 0
        const FLAGS_NONE = 0;
        /// Force 8-byte align: 1 << 7
        const ALIGN_8 = 128;
        /// Forced 16-byte align: 1 << 8
        const ALIGN_16 = 256;
        /// Not owned by class: 1 << 9
        const NOT_OWNED = 512;
        /// Skip serializing: 1 << 10
        const SERIALIZE_IGNORED = 1024;
    }
}

impl Serialize for FlagValues {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sv = serializer.serialize_enum_flags()?;
        if self.is_empty() {
            sv.serialize_empty_bit()?;
            return sv.end();
        };

        for flag in self.iter() {
            match flag {
                FlagValues::FLAGS_NONE => sv.serialize_field("FLAGS_NONE", &FlagValues::FLAGS_NONE),
                FlagValues::ALIGN_8 => sv.serialize_field("ALIGN_8", &FlagValues::ALIGN_8),
                FlagValues::ALIGN_16 => sv.serialize_field("ALIGN_16", &FlagValues::ALIGN_16),
                FlagValues::NOT_OWNED => sv.serialize_field("NOT_OWNED", &FlagValues::NOT_OWNED),
                FlagValues::SERIALIZE_IGNORED => {
                    sv.serialize_field("SERIALIZE_IGNORED", &FlagValues::SERIALIZE_IGNORED)
                }
                remain => sv.serialize_field(&remain.bits().to_string(), &remain.bits()),
            }?
        }

        // For binary
        sv.serialize_bits(&self.bits())?;
        sv.end()
    }
}
