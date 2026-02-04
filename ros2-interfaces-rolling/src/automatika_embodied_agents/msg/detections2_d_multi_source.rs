use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Detections2DMultiSource {
    pub header: crate::std_msgs::msg::Header,
    pub detections: Vec<crate::automatika_embodied_agents::msg::Detections2D>,
}

impl Default for Detections2DMultiSource {
    fn default() -> Self {
        Detections2DMultiSource {
            header: crate::std_msgs::msg::Header::default(),
            detections: Vec::new(),
        }
    }
}

impl ros2_client::Message for Detections2DMultiSource {}
