//! Deserializing each field element in an `Struct`

use super::{
    parser::tag::{end_tag, field_start_close_tag, field_start_open_tag},
    XmlDeserializer,
};
use crate::{
    errors::de::{Error, Result},
    tri,
    xml::de::parser::tag::{array_field_start_close_tag, class_start_tag, start_tag},
};
use havok_serde::de::{DeserializeSeed, MapAccess};
use havok_types::Pointer;

/// A structure for deserializing each element in an `Struct`.
///
/// # Expected XML
/// ```xml
/// <hkobject name="#0010" class="hkbProjectData" signature="0x13a39ba7"> <!-- <-Parsed by `deserialize_struct` -->
///   <!-- memSizeAndFlags SERIALIZE_IGNORED -->
///   <!-- referenceCount SERIALIZE_IGNORED -->
///   <hkparam name="worldUpWS">(0.000000 0.000000 1.000000 0.000000)</hkparam>
///   <hkparam name="stringData">#0009</hkparam>
///   <hkparam name="defaultEventMode">EVENT_MODE_IGNORE_FROM_GENERATOR</hkparam>
/// </hkobject>
/// ```
#[derive(Debug)]
pub struct MapDeserializer<'a, 'de: 'a> {
    /// Deserializer
    de: &'a mut XmlDeserializer<'de>,
    class_name: &'static str,
    fields: &'static [&'static str],

    /// To check fields length
    index: usize,
    ptr_name: Option<Pointer>,

    ///  In `Struct` deserialization?
    ///
    /// # Why need this flag?
    /// This flag is necessary because XML handles deserialization of a field in a struct differently
    /// than it handles deserialization of a struct in a field in a struct.
    ///
    /// - root struct: `<hkobject name="#0050" class="" signature=""></hkobject>`
    /// - struct in field: `<hkobject></hkobject>`
    in_struct: bool,
}

impl<'a, 'de> MapDeserializer<'a, 'de> {
    /// Create a new map deserializer
    pub fn new(
        de: &'a mut XmlDeserializer<'de>,
        class_name: &'static str,
        fields: &'static [&'static str],
    ) -> Self {
        Self {
            de,
            class_name,
            fields,
            index: 0,
            ptr_name: None,
            in_struct: false,
        }
    }
}

impl<'a, 'de> MapDeserializer<'a, 'de> {
    fn parse_root_class_tag(&mut self) -> Result<()> {
        if self.in_struct {
            // When a struct is present in the field of struct, the name and signature attributes are not present.
            tri!(self.de.parse(start_tag("hkobject")));
        } else {
            let (ptr_name, class_name, signature) = tri!(self.de.parse(class_start_tag()));
            #[cfg(feature = "tracing")]
            tracing::debug!("ptr_name={ptr_name}, class_name={class_name}, Signature={signature}");

            if self.class_name != class_name {
                return Err(Error::MismatchClassName {
                    actual: self.class_name,
                    expected: class_name.to_string(),
                });
            };
            self.in_struct = true;
        }

        Ok(())
    }
}

impl<'a, 'de> MapAccess<'de> for MapDeserializer<'a, 'de> {
    type Error = Error;

    #[inline]
    fn class_ptr(&mut self) -> Option<Pointer> {
        self.ptr_name
    }

    // Parse e.g. `<hkparam name="worldUpWS">`
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.index == 0 {
            tri!(self.parse_root_class_tag());
        };

        // Check if there are no more elements.
        if self.de.parse_peek(end_tag("hkobject")).is_ok() && self.fields.len() == self.index {
            self.in_struct = false;
            return Ok(None);
        }

        tri!(self.de.parse(field_start_open_tag())); // Parse `<hkparam name=`
        let key = seed.deserialize(&mut *self.de).map(Some); // Deserialize a map key.
        tri!(self.de.parse(field_start_close_tag())); // Parse `>`
        self.index += 1;

        key
    }

    // Parse e.g. `<hkparam name="boneWeights" numelements="90">`
    fn next_array_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.index == 0 {
            tri!(self.parse_root_class_tag());
        };

        // Check if there are no more elements.
        if self.de.parse_peek(end_tag("hkobject")).is_ok() && self.fields.len() == self.index {
            return Ok(None);
        }

        tri!(self.de.parse(field_start_open_tag())); // Parse `<hkparam name=`
        let key = seed.deserialize(&mut *self.de).map(Some);
        let _len = tri!(self.de.parse(array_field_start_close_tag())); // Parse ` numelements="3">`
        #[cfg(feature = "tracing")]
        tracing::debug!(_len);

        self.index += 1;
        key
    }

    // Parse e.g. `(0.000000 0.000000 1.000000 0.000000)</hkparam>`
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = tri!(seed.deserialize(&mut *self.de));
        tri!(self.de.parse(end_tag("hkparam")));
        Ok(value)
    }
}
