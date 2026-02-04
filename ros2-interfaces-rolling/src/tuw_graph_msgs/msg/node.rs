use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: i64,
    pub valid: bool,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub flags: Vec<u32>,
}

impl Node {
    pub const FLAG_NA: u32 = 0;
    pub const FLAG_UNTOUCHED: u32 = 1;
    pub const FLAG_OPEN: u32 = 2;
    pub const FLAG_CLOSED: u32 = 3;
    pub const FLAG_VISITED: u32 = 4;
    pub const FLAG_START: u32 = 5;
    pub const FLAG_GOAL: u32 = 6;
}

impl Default for Node {
    fn default() -> Self {
        Node {
            id: 0,
            valid: false,
            pose: crate::geometry_msgs::msg::Pose::default(),
            flags: Vec::new(),
        }
    }
}

impl ros2_client::Message for Node {}
