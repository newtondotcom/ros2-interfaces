use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinishTrajectoryReq {
    pub trajectory_id: i32,
}

impl Default for FinishTrajectoryReq {
    fn default() -> Self {
        FinishTrajectoryReq {
            trajectory_id: 0,
        }
    }
}

impl ros2_client::Message for FinishTrajectoryReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinishTrajectoryRes {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
}

impl Default for FinishTrajectoryRes {
    fn default() -> Self {
        FinishTrajectoryRes {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
        }
    }
}

impl ros2_client::Message for FinishTrajectoryRes {}


pub struct FinishTrajectory;
impl ros2_client::Service for FinishTrajectory {
    type Request = FinishTrajectoryReq;
    type Response = FinishTrajectoryRes;

    fn request_type_name(&self) -> &str { "FinishTrajectoryReq" }
    fn response_type_name(&self) -> &str { "FinishTrajectoryRes" }
}
