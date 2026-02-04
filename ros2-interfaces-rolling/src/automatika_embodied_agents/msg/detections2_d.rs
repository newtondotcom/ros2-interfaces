use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Detections2D {
    pub header: crate::std_msgs::msg::Header,
    pub scores: Vec<f64>,
    pub labels: Vec<::std::string::String>,
    pub boxes: Vec<crate::automatika_embodied_agents::msg::Bbox2D>,
    pub image: crate::sensor_msgs::msg::Image,
    pub compressed_image: crate::sensor_msgs::msg::CompressedImage,
    pub depth: crate::sensor_msgs::msg::Image,
}

impl Default for Detections2D {
    fn default() -> Self {
        Detections2D {
            header: crate::std_msgs::msg::Header::default(),
            scores: Vec::new(),
            labels: Vec::new(),
            boxes: Vec::new(),
            image: crate::sensor_msgs::msg::Image::default(),
            compressed_image: crate::sensor_msgs::msg::CompressedImage::default(),
            depth: crate::sensor_msgs::msg::Image::default(),
        }
    }
}

impl ros2_client::Message for Detections2D {}
