use crate::ClassMap;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Avoid number key error
///
/// See: https://github.com/toml-lang/toml/issues/1001
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct ClassMapWrapper<'a> {
    #[serde_as(as = "Vec<(DisplayFromStr, _)>")]
    inner: Vec<(usize, havok_classes::Classes<'a>)>,
}

impl<'a> ClassMapWrapper<'a> {
    #[inline]
    pub fn new(map: ClassMap<'a>) -> Self {
        ClassMapWrapper {
            inner: map.into_iter().collect(),
        }
    }
}

impl<'a> From<ClassMap<'a>> for ClassMapWrapper<'a> {
    #[inline]
    fn from(map: ClassMap<'a>) -> Self {
        ClassMapWrapper::new(map)
    }
}

impl<'a> From<ClassMapWrapper<'a>> for ClassMap<'a> {
    #[inline]
    fn from(wrapper: ClassMapWrapper<'a>) -> Self {
        wrapper.inner.into_iter().collect()
    }
}
