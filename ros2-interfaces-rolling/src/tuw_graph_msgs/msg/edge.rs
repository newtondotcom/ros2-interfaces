use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub id: i64,
    pub valid: bool,
    pub weight: f64,
    pub flags: Vec<u32>,
    pub start: i64,
    pub end: i64,
    pub path: Vec<crate::geometry_msgs::msg::Pose>,
}

impl Edge {
    pub const FLAG_NA: u32 = 0;
    pub const FLAG_UNTOUCHED: u32 = 1;
    pub const FLAG_OPEN: u32 = 2;
    pub const FLAG_CLOSED: u32 = 3;
    pub const FLAG_VISITED: u32 = 4;
}

impl Default for Edge {
    fn default() -> Self {
        Edge {
            id: 0,
            valid: false,
            weight: 0.0,
            flags: Vec::new(),
            start: 0,
            end: 0,
            path: Vec::new(),
        }
    }
}

impl ros2_client::Message for Edge {}
