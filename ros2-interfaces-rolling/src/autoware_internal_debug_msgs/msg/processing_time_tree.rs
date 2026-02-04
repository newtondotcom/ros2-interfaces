use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingTimeTree {
    pub nodes: Vec<crate::autoware_internal_debug_msgs::msg::ProcessingTimeNode>,
}

impl Default for ProcessingTimeTree {
    fn default() -> Self {
        ProcessingTimeTree {
            nodes: Vec::new(),
        }
    }
}

impl ros2_client::Message for ProcessingTimeTree {}
