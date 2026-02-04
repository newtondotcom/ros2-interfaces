use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathPointWithLaneId {
    pub point: crate::autoware_planning_msgs::msg::PathPoint,
    pub lane_ids: Vec<i64>,
}

impl Default for PathPointWithLaneId {
    fn default() -> Self {
        PathPointWithLaneId {
            point: crate::autoware_planning_msgs::msg::PathPoint::default(),
            lane_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for PathPointWithLaneId {}
