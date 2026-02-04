use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Graph {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub nodes: Vec<crate::rosgraph_monitor_msgs::msg::NodeInfo>,
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            nodes: Vec::new(),
        }
    }
}

impl ros2_client::Message for Graph {}
