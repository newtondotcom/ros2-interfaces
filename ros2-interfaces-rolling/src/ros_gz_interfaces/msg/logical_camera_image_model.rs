use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogicalCameraImageModel {
    pub name: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for LogicalCameraImageModel {
    fn default() -> Self {
        LogicalCameraImageModel {
            name: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for LogicalCameraImageModel {}
