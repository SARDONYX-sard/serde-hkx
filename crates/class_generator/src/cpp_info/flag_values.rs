//! Flags for field alignment needs, skipping serialization, etc.
use havok_types::impl_str_serde;
use std::{borrow::Cow, str::FromStr};

bitflags::bitflags! {
    /// Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++).
    ///
    /// # On XML
    /// When all bits are 0, "0" is inserted.
    /// (Even if `FLAGS_NONE = 0` and 0 is replaced by `FLAGS_NONE`, "0" will appear when reconverting xml -> hkx -> xml.)
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Default for FlagValues {
    fn default() -> Self {
        Self::empty()
    }
}

impl_str_serde!(FlagValues);

impl FromStr for FlagValues {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "0" {
            return Ok(FlagValues::empty());
        }

        let mut flags = FlagValues::empty();
        for token in s.split('|') {
            match token.trim() {
                "FLAGS_NONE" => flags |= FlagValues::FLAGS_NONE,
                "ALIGN_8" => flags |= FlagValues::ALIGN_8,
                "ALIGN_16" => flags |= FlagValues::ALIGN_16,
                "NOT_OWNED" => flags |= FlagValues::NOT_OWNED,
                "SERIALIZE_IGNORED" => flags |= FlagValues::SERIALIZE_IGNORED,
                unknown => {
                    let bits = parse_int::parse(unknown).map_err(|_| {
                        format!("Expected FlagValues or bits, but it is not number: '{unknown}'",)
                    })?;
                    flags |= FlagValues::from_bits_retain(bits);
                }
            }
        }

        Ok(flags)
    }
}

impl core::fmt::Display for FlagValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_empty() {
            return write!(f, "0");
        }

        let mut flags: Vec<Cow<'_, str>> = Vec::new();
        for flag in self.iter() {
            match flag {
                Self::FLAGS_NONE => flags.push("FLAGS_NONE".into()),
                Self::ALIGN_8 => flags.push("ALIGN_8".into()),
                Self::ALIGN_16 => flags.push("ALIGN_16".into()),
                Self::NOT_OWNED => flags.push("NOT_OWNED".into()),
                Self::SERIALIZE_IGNORED => flags.push("SERIALIZE_IGNORED".into()),
                remain => flags.push(remain.bits().to_string().into()),
            };
        }

        write!(f, "{}", flags.join("|"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn flags_to_str() {
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
    fn str_to_flags() {
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
            Ok(FlagValues::FLAGS_NONE
                | FlagValues::ALIGN_8
                | FlagValues::ALIGN_16
                | FlagValues::SERIALIZE_IGNORED
                | FlagValues::from_bits_retain(64)),
            "Unknown flags should be able to accepted."
        );
    }
}
