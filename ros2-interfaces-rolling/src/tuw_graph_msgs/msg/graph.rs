use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Graph {
    pub header: crate::std_msgs::msg::Header,
    pub origin: crate::geometry_msgs::msg::Pose,
    pub nodes: Vec<crate::tuw_graph_msgs::msg::Node>,
    pub edges: Vec<crate::tuw_graph_msgs::msg::Edge>,
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            header: crate::std_msgs::msg::Header::default(),
            origin: crate::geometry_msgs::msg::Pose::default(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl ros2_client::Message for Graph {}
