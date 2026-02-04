use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Int64Stamped {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub data: i64,
}

impl Default for Int64Stamped {
    fn default() -> Self {
        Int64Stamped {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            data: 0,
        }
    }
}

impl ros2_client::Message for Int64Stamped {}
