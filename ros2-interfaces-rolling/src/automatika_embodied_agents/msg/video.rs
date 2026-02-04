use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Video {
    pub header: crate::std_msgs::msg::Header,
    pub frames: Vec<crate::sensor_msgs::msg::Image>,
    pub compressed_frames: Vec<crate::sensor_msgs::msg::CompressedImage>,
}

impl Default for Video {
    fn default() -> Self {
        Video {
            header: crate::std_msgs::msg::Header::default(),
            frames: Vec::new(),
            compressed_frames: Vec::new(),
        }
    }
}

impl ros2_client::Message for Video {}
