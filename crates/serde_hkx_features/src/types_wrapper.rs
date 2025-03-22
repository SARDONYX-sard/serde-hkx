//! For Json, Toml, Yaml

use crate::ClassMap;
use havok_classes::Classes;
use havok_types::Pointer;
use indexmap::IndexMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// - key: class index(e.g `#0001`)
/// - value: C++ Class
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ClassPtrMap<'a> {
    /// Path of Json schema
    #[cfg_attr(feature = "json_schema", schemars(rename = "$schema"))]
    #[serde(default = "default_schema_url")]
    schema: Cow<'a, str>,
    #[serde(flatten)]
    #[serde(borrow)]
    classes: IndexMap<Pointer<'a>, Classes<'a>>,
}

#[inline]
const fn default_schema_url() -> Cow<'static, str> {
    Cow::Borrowed(
        "https://raw.githubusercontent.com/SARDONYX-sard/serde-hkx/refs/tags/0.7.0/assets/serde_hkx_schema.json",
    )
}

impl<'a> ClassPtrMap<'a> {
    pub fn from_class_map(value: ClassMap<'a>) -> Self {
        Self {
            schema: default_schema_url(),
            classes: value
                .into_par_iter()
                .map(|(k, v)| (Pointer::new(k), v))
                .collect(),
        }
    }

    pub fn into_class_map(self) -> ClassMap<'a> {
        self.classes
            .into_par_iter()
            .map(|(k, v)| (k.get(), v))
            .collect()
    }
}
