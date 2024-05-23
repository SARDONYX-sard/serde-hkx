use crate::ser::Serialize;

#[allow(unused)]
pub enum EventMode {
    EventModeDefault = 0,
    EventModeProcessAll = 1,
    EventModeIgnoreFromGenerator = 2,
    EventModeIgnoreToGenerator = 3,
}

impl Serialize for EventMode {
    fn serialize<S: crate::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use crate::ser::SerializeFlags;

        let mut sv = serializer.serialize_flags()?;

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
