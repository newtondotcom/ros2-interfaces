use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryQueryReq {
    pub trajectory_id: i32,
}

impl Default for TrajectoryQueryReq {
    fn default() -> Self {
        TrajectoryQueryReq {
            trajectory_id: 0,
        }
    }
}

impl ros2_client::Message for TrajectoryQueryReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryQueryRes {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
    pub trajectory: Vec<crate::geometry_msgs::msg::PoseStamped>,
}

impl Default for TrajectoryQueryRes {
    fn default() -> Self {
        TrajectoryQueryRes {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
            trajectory: Vec::new(),
        }
    }
}

impl ros2_client::Message for TrajectoryQueryRes {}


pub struct TrajectoryQuery;
impl ros2_client::Service for TrajectoryQuery {
    type Request = TrajectoryQueryReq;
    type Response = TrajectoryQueryRes;

    fn request_type_name(&self) -> &str { "TrajectoryQueryReq" }
    fn response_type_name(&self) -> &str { "TrajectoryQueryRes" }
}
