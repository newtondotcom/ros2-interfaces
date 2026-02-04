use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Goals {
    pub header: crate::std_msgs::msg::Header,
    pub goals: Vec<crate::geometry_msgs::msg::PoseStamped>,
}

impl Default for Goals {
    fn default() -> Self {
        Goals {
            header: crate::std_msgs::msg::Header::default(),
            goals: Vec::new(),
        }
    }
}

impl ros2_client::Message for Goals {}
