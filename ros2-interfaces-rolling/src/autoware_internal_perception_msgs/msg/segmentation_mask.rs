use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentationMask {
    pub header: crate::std_msgs::msg::Header,
    pub classification: Vec<crate::autoware_perception_msgs::msg::ObjectClassification>,
    pub image: crate::sensor_msgs::msg::Image,
}

impl Default for SegmentationMask {
    fn default() -> Self {
        SegmentationMask {
            header: crate::std_msgs::msg::Header::default(),
            classification: Vec::new(),
            image: crate::sensor_msgs::msg::Image::default(),
        }
    }
}

impl ros2_client::Message for SegmentationMask {}
