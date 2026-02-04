use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Int32Stamped {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub data: i32,
}

impl Default for Int32Stamped {
    fn default() -> Self {
        Int32Stamped {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            data: 0,
        }
    }
}

impl ros2_client::Message for Int32Stamped {}
