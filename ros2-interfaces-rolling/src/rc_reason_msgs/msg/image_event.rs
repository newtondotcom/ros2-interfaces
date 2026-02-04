use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageEvent {
    pub header: crate::std_msgs::msg::Header,
    pub imageset_stamp: crate::builtin_interfaces::msg::Time,
}

impl Default for ImageEvent {
    fn default() -> Self {
        ImageEvent {
            header: crate::std_msgs::msg::Header::default(),
            imageset_stamp: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for ImageEvent {}
