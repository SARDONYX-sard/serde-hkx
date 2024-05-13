//! Flags for field alignment needs, skipping serialization, etc.
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    /// Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++).
    ///
    /// # On XML
    /// When all bits are 0, "0" is inserted.
    /// (Even if `FLAGS_NONE = 0` and 0 is replaced by `FLAGS_NONE`, "0" will appear when reconverting xml -> hkx -> xml.)
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SerializeDisplay, DeserializeFromStr)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn to_string() {
        assert_eq!(
            FlagValues::FLAGS_NONE.to_string(),
            "0",
            "If the bit is empty, `0` should be displayed"
        );

        assert_eq!(
            (FlagValues::ALIGN_8 | FlagValues::ALIGN_16 | FlagValues::SERIALIZE_IGNORED)
                .to_string(),
            "ALIGN_8|ALIGN_16|SERIALIZE_IGNORED",
            "`FLAGS_NONE` will not be included unless it is manually included in the flag type."
        );
    }

    #[test]
    fn from_str() {
        assert_eq!("0".parse(), Ok(FlagValues::FLAGS_NONE));
        assert_eq!("FLAGS_NONE".parse(), Ok(FlagValues::FLAGS_NONE));

        assert_eq!(
            "ALIGN_8|ALIGN_16|SERIALIZE_IGNORED".parse(),
            Ok(FlagValues::FLAGS_NONE
                | FlagValues::ALIGN_8
                | FlagValues::ALIGN_16
                | FlagValues::SERIALIZE_IGNORED),
            "FLAGS_NONE must be held at any time."
        );

        assert_eq!(
            "ALIGN_8|ALIGN_16|SERIALIZE_IGNORED|64".parse(),
            Ok(FlagValues::ALIGN_8
                | FlagValues::ALIGN_16
                | FlagValues::SERIALIZE_IGNORED
                | FlagValues::from_bits_retain(64)),
            "Unknown flags should be able to accepted."
        );
    }
}
