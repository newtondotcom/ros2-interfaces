use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Float64MultiArrayStamped {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub layout: crate::std_msgs::msg::MultiArrayLayout,
    pub data: Vec<f64>,
}

impl Default for Float64MultiArrayStamped {
    fn default() -> Self {
        Float64MultiArrayStamped {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            layout: crate::std_msgs::msg::MultiArrayLayout::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for Float64MultiArrayStamped {}
