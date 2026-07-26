pub mod ser {
    //! Serialize ClassMap with extra formats.
    use crate::convert::Format;
    use crate::error::Result;
    use crate::serde::ser::{JsonSnafu, TomlSnafu};
    use crate::types_wrapper::ClassPtrMap;
    use snafu::ResultExt as _;

    /// Serialize bytes(file contents) to a file.
    ///
    /// # Errors
    /// If the information required for serialization is missing.
    ///
    /// See `serde_hkx::errors::ser::Error` for possible errors that may occur.
    ///
    /// # Panics
    /// If the `OutFormat` is not `json`, `toml` and `yaml`.
    /// That means the API is being used incorrectly.
    pub fn to_bytes(
        classes: &mut ClassPtrMap<'_>,
        output_format: Format,
    ) -> Result<Vec<u8>, crate::serde::ser::SerError> {
        let contents = match output_format {
            Format::Json => sonic_rs::to_string_pretty(&classes).context(JsonSnafu {})?,
            Format::Toml => basic_toml::to_string(&classes).context(TomlSnafu {})?,
            _ => unreachable!(),
        };
        Ok(contents.into_bytes())
    }
}
