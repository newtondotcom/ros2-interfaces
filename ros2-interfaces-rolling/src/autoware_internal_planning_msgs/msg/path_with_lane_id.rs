use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathWithLaneId {
    pub header: crate::std_msgs::msg::Header,
    pub points: Vec<crate::autoware_internal_planning_msgs::msg::PathPointWithLaneId>,
    pub left_bound: Vec<crate::geometry_msgs::msg::Point>,
    pub right_bound: Vec<crate::geometry_msgs::msg::Point>,
}

impl Default for PathWithLaneId {
    fn default() -> Self {
        PathWithLaneId {
            header: crate::std_msgs::msg::Header::default(),
            points: Vec::new(),
            left_bound: Vec::new(),
            right_bound: Vec::new(),
        }
    }
}

impl ros2_client::Message for PathWithLaneId {}
