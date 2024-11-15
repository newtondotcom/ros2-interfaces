use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTrajectoryStatesReq {

}

impl Default for GetTrajectoryStatesReq {
    fn default() -> Self {
        GetTrajectoryStatesReq {

        }
    }
}

impl ros2_client::Message for GetTrajectoryStatesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTrajectoryStatesRes {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
    pub trajectory_states: crate::cartographer_ros_msgs::msg::TrajectoryStates,
}

impl Default for GetTrajectoryStatesRes {
    fn default() -> Self {
        GetTrajectoryStatesRes {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
            trajectory_states: crate::cartographer_ros_msgs::msg::TrajectoryStates::default(),
        }
    }
}

impl ros2_client::Message for GetTrajectoryStatesRes {}


pub struct GetTrajectoryStates;
impl ros2_client::Service for GetTrajectoryStates {
    type Request = GetTrajectoryStatesReq;
    type Response = GetTrajectoryStatesRes;

    fn request_type_name(&self) -> &str { "GetTrajectoryStatesReq" }
    fn response_type_name(&self) -> &str { "GetTrajectoryStatesRes" }
}
