use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trajectory {
    pub header: crate::std_msgs::msg::Header,
    pub x0: f32,
    pub y0: f32,
    pub heading0: f32,
    #[serde(rename = "type")]    pub type_: u8,
    pub nodes: Vec<crate::ackermann_nlmpc_msgs::msg::TrajectoryNode>,
}

impl Default for Trajectory {
    fn default() -> Self {
        Trajectory {
            header: crate::std_msgs::msg::Header::default(),
            x0: 0.0,
            y0: 0.0,
            heading0: 0.0,
            type_: 0,
            nodes: Vec::new(),
        }
    }
}

impl ros2_client::Message for Trajectory {}
