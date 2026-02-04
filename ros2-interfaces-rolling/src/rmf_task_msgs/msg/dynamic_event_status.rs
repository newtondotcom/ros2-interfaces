use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicEventStatus {
    pub dynamic_event_seq: u32,
    pub dynamic_state: u8,
    pub status: ::std::string::String,
    pub id: u64,
    pub time: crate::builtin_interfaces::msg::Time,
}

impl DynamicEventStatus {
    pub const DYNAMIC_STATE_INACTIVE: u8 = 0;
    pub const DYNAMIC_STATE_WAITING: u8 = 1;
    pub const DYNAMIC_STATE_RUNNING: u8 = 2;
}

impl Default for DynamicEventStatus {
    fn default() -> Self {
        DynamicEventStatus {
            dynamic_event_seq: 0,
            dynamic_state: 0,
            status: ::std::string::String::new(),
            id: 0,
            time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for DynamicEventStatus {}
