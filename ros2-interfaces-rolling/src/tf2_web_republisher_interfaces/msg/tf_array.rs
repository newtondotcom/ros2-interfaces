use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TFArray {
    pub transforms: Vec<crate::geometry_msgs::msg::TransformStamped>,
}

impl Default for TFArray {
    fn default() -> Self {
        TFArray {
            transforms: Vec::new(),
        }
    }
}

impl ros2_client::Message for TFArray {}
