use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoolStamped {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub data: bool,
}

impl Default for BoolStamped {
    fn default() -> Self {
        BoolStamped {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            data: false,
        }
    }
}

impl ros2_client::Message for BoolStamped {}
