use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackingsMultiSource {
    pub header: crate::std_msgs::msg::Header,
    pub trackings: Vec<crate::automatika_embodied_agents::msg::Trackings>,
}

impl Default for TrackingsMultiSource {
    fn default() -> Self {
        TrackingsMultiSource {
            header: crate::std_msgs::msg::Header::default(),
            trackings: Vec::new(),
        }
    }
}

impl ros2_client::Message for TrackingsMultiSource {}
