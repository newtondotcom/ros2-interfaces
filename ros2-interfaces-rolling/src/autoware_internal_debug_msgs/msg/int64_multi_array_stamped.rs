use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Int64MultiArrayStamped {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub layout: crate::std_msgs::msg::MultiArrayLayout,
    pub data: Vec<i64>,
}

impl Default for Int64MultiArrayStamped {
    fn default() -> Self {
        Int64MultiArrayStamped {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            layout: crate::std_msgs::msg::MultiArrayLayout::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for Int64MultiArrayStamped {}
