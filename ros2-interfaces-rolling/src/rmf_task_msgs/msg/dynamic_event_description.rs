use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicEventDescription {
    pub fleet: ::std::string::String,
    pub robot: ::std::string::String,
    pub dynamic_event_seq: u32,
    pub description: ::std::string::String,
    pub start_time: crate::builtin_interfaces::msg::Time,
}

impl Default for DynamicEventDescription {
    fn default() -> Self {
        DynamicEventDescription {
            fleet: ::std::string::String::new(),
            robot: ::std::string::String::new(),
            dynamic_event_seq: 0,
            description: ::std::string::String::new(),
            start_time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for DynamicEventDescription {}
