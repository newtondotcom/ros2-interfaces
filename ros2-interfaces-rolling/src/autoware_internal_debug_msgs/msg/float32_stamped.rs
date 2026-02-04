use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Float32Stamped {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub data: f32,
}

impl Default for Float32Stamped {
    fn default() -> Self {
        Float32Stamped {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            data: 0.0,
        }
    }
}

impl ros2_client::Message for Float32Stamped {}
