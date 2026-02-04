use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogicalCameraImage {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub model: Vec<crate::ros_gz_interfaces::msg::LogicalCameraImageModel>,
}

impl Default for LogicalCameraImage {
    fn default() -> Self {
        LogicalCameraImage {
            header: crate::std_msgs::msg::Header::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            model: Vec::new(),
        }
    }
}

impl ros2_client::Message for LogicalCameraImage {}
