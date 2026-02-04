use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlexibleTimeReservationAlt {
    pub resource_name: ::std::string::String,
    pub cost: f64,
    pub start_time: crate::rmf_reservation_msgs::msg::StartTimeRange,
    pub has_end: bool,
    pub duration: crate::builtin_interfaces::msg::Duration,
}

impl Default for FlexibleTimeReservationAlt {
    fn default() -> Self {
        FlexibleTimeReservationAlt {
            resource_name: ::std::string::String::new(),
            cost: 0.0,
            start_time: crate::rmf_reservation_msgs::msg::StartTimeRange::default(),
            has_end: false,
            duration: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for FlexibleTimeReservationAlt {}
