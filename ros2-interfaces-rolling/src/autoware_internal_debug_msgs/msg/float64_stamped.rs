use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Float64Stamped {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub data: f64,
}

impl Default for Float64Stamped {
    fn default() -> Self {
        Float64Stamped {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            data: 0.0,
        }
    }
}

impl ros2_client::Message for Float64Stamped {}
