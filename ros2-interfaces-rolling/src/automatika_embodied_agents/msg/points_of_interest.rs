use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointsOfInterest {
    pub header: crate::std_msgs::msg::Header,
    pub points: Vec<crate::automatika_embodied_agents::msg::Point2D>,
    pub image: crate::sensor_msgs::msg::Image,
    pub compressed_image: crate::sensor_msgs::msg::CompressedImage,
    pub depth: crate::sensor_msgs::msg::Image,
}

impl Default for PointsOfInterest {
    fn default() -> Self {
        PointsOfInterest {
            header: crate::std_msgs::msg::Header::default(),
            points: Vec::new(),
            image: crate::sensor_msgs::msg::Image::default(),
            compressed_image: crate::sensor_msgs::msg::CompressedImage::default(),
            depth: crate::sensor_msgs::msg::Image::default(),
        }
    }
}

impl ros2_client::Message for PointsOfInterest {}
