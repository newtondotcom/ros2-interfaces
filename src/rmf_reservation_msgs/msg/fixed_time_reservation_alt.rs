use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FixedTimeReservationAlt {
    pub resource_name: ::std::string::String,
    pub cost: f64,
    pub start_time: crate::builtin_interfaces::msg::Time,
    pub has_end: bool,
    pub duration: crate::builtin_interfaces::msg::Duration,
}

impl Default for FixedTimeReservationAlt {
    fn default() -> Self {
        FixedTimeReservationAlt {
            resource_name: ::std::string::String::new(),
            cost: 0.0,
            start_time: crate::builtin_interfaces::msg::Time::default(),
            has_end: false,
            duration: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for FixedTimeReservationAlt {}
