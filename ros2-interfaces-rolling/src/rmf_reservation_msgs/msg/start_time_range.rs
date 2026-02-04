use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartTimeRange {
    pub earliest_start_time: crate::builtin_interfaces::msg::Time,
    pub has_earliest_start_time: bool,
    pub latest_start_time: crate::builtin_interfaces::msg::Time,
    pub has_latest_start_time: bool,
}

impl Default for StartTimeRange {
    fn default() -> Self {
        StartTimeRange {
            earliest_start_time: crate::builtin_interfaces::msg::Time::default(),
            has_earliest_start_time: false,
            latest_start_time: crate::builtin_interfaces::msg::Time::default(),
            has_latest_start_time: false,
        }
    }
}

impl ros2_client::Message for StartTimeRange {}
