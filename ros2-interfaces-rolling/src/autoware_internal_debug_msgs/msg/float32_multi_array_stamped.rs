use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Float32MultiArrayStamped {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub layout: crate::std_msgs::msg::MultiArrayLayout,
    pub data: Vec<f32>,
}

impl Default for Float32MultiArrayStamped {
    fn default() -> Self {
        Float32MultiArrayStamped {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            layout: crate::std_msgs::msg::MultiArrayLayout::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for Float32MultiArrayStamped {}
