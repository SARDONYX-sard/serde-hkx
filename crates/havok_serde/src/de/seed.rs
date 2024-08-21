// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// The following code was written by modifying serde ver. 1.0.202.
// See: https://github.com/serde-rs/serde/commit/58b3af4c2915c3ae789778a11f3b7a468c1cec17
//
// And serde holds the same license as Rust. https://github.com/rust-lang/rust/pull/43498
use crate::de::{Deserialize, DeserializeSeed, Deserializer};

/// A DeserializeSeed helper for implementing deserialize_in_place Visitors.
///
/// Wraps a mutable reference and calls deserialize_in_place on it.
pub struct InPlaceSeed<'a, T: 'a>(pub &'a mut T);

impl<'a, 'de, T> DeserializeSeed<'de> for InPlaceSeed<'a, T>
where
    T: Deserialize<'de>,
{
    type Value = ();
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize_in_place(deserializer, self.0)
    }
}
