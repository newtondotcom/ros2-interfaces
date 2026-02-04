use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryNode {
    pub header: crate::std_msgs::msg::Header,
    pub x: f64,
    pub y: f64,
    pub heading: f64,
    pub v: f64,
    pub a: f64,
    pub delta: f64,
    pub beta: f64,
    pub mode: u8,
    pub pw_l: f64,
    pub pw_r: f64,
}

impl Default for TrajectoryNode {
    fn default() -> Self {
        TrajectoryNode {
            header: crate::std_msgs::msg::Header::default(),
            x: 0.0,
            y: 0.0,
            heading: 0.0,
            v: 0.0,
            a: 0.0,
            delta: 0.0,
            beta: 0.0,
            mode: 0,
            pw_l: 0.0,
            pw_r: 0.0,
        }
    }
}

impl ros2_client::Message for TrajectoryNode {}
