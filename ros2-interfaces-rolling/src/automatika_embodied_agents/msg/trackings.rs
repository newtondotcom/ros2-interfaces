use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trackings {
    pub header: crate::std_msgs::msg::Header,
    pub centroids: Vec<crate::automatika_embodied_agents::msg::Point2D>,
    pub labels: Vec<::std::string::String>,
    pub boxes: Vec<crate::automatika_embodied_agents::msg::Bbox2D>,
    pub ids: Vec<i8>,
    pub estimated_velocities: Vec<crate::automatika_embodied_agents::msg::Point2D>,
    pub image: crate::sensor_msgs::msg::Image,
    pub compressed_image: crate::sensor_msgs::msg::CompressedImage,
    pub depth: crate::sensor_msgs::msg::Image,
}

impl Default for Trackings {
    fn default() -> Self {
        Trackings {
            header: crate::std_msgs::msg::Header::default(),
            centroids: Vec::new(),
            labels: Vec::new(),
            boxes: Vec::new(),
            ids: Vec::new(),
            estimated_velocities: Vec::new(),
            image: crate::sensor_msgs::msg::Image::default(),
            compressed_image: crate::sensor_msgs::msg::CompressedImage::default(),
            depth: crate::sensor_msgs::msg::Image::default(),
        }
    }
}

impl ros2_client::Message for Trackings {}
