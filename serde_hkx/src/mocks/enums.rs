use havok_serde::ser::{Serialize, SerializeFlags, Serializer};

#[allow(unused)]
#[derive(Debug, Clone, Default, PartialEq)]
pub enum EventMode {
    #[default]
    EventModeDefault = 0,
    EventModeProcessAll = 1,
    EventModeIgnoreFromGenerator = 2,
    EventModeIgnoreToGenerator = 3,
}

impl Serialize for EventMode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sv = serializer.serialize_enum_flags()?;

        match self {
            EventMode::EventModeDefault => sv.serialize_field("EVENT_MODE_DEFAULT", &0),
            EventMode::EventModeProcessAll => sv.serialize_field("EVENT_MODE_PROCESS_ALL", &1),
            EventMode::EventModeIgnoreFromGenerator => {
                sv.serialize_field("EVENT_MODE_IGNORE_FROM_GENERATOR", &2)
            }
            EventMode::EventModeIgnoreToGenerator => {
                sv.serialize_field("EVENT_MODE_IGNORE_TO_GENERATOR", &3)
            }
        }?;
        sv.end()
    }
}