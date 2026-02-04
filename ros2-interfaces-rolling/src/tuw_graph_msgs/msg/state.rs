use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub step: i64,
    pub list_node_open: Vec<i32>,
    pub list_edge_open: Vec<i32>,
    pub color_open: Vec<crate::std_msgs::msg::ColorRGBA>,
    pub list_node_closed: Vec<i32>,
    pub list_edge_closed: Vec<i32>,
    pub color_closed: Vec<crate::std_msgs::msg::ColorRGBA>,
}

impl Default for State {
    fn default() -> Self {
        State {
            step: 0,
            list_node_open: Vec::new(),
            list_edge_open: Vec::new(),
            color_open: Vec::new(),
            list_node_closed: Vec::new(),
            list_edge_closed: Vec::new(),
            color_closed: Vec::new(),
        }
    }
}

impl ros2_client::Message for State {}
