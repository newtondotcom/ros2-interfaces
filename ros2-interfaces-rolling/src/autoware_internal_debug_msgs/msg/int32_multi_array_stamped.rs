use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Int32MultiArrayStamped {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub layout: crate::std_msgs::msg::MultiArrayLayout,
    pub data: Vec<i32>,
}

impl Default for Int32MultiArrayStamped {
    fn default() -> Self {
        Int32MultiArrayStamped {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            layout: crate::std_msgs::msg::MultiArrayLayout::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for Int32MultiArrayStamped {}
