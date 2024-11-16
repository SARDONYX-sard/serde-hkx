//! For Json, Toml, Yaml

use crate::ClassMap;
use havok_classes::Classes;
use havok_types::Pointer;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// - key: class index(e.g `#0001`)
/// - value: C++ Class
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ClassPtrMap<'a> {
    /// Path of Json schema
    #[cfg_attr(feature = "json_schema", schemars(rename = "$schema"))]
    #[serde(default)]
    schema: Option<String>,
    #[serde(flatten)]
    classes: IndexMap<Pointer, Classes<'a>>,
}

impl<'a> ClassPtrMap<'a> {
    pub fn from_class_map(value: ClassMap<'a>) -> Self {
        Self {
            schema: None,
            classes: value
                .into_iter()
                .map(|(k, v)| (Pointer::from(k), v))
                .collect(),
        }
    }

    pub fn into_class_map(self) -> ClassMap<'a> {
        self.classes
            .into_iter()
            .map(|(k, v)| (k.get(), v))
            .collect()
    }
}
