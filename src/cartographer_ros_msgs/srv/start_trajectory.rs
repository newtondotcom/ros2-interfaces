use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartTrajectoryReq {
    pub configuration_directory: ::std::string::String,
    pub configuration_basename: ::std::string::String,
    pub use_initial_pose: bool,
    pub initial_pose: crate::geometry_msgs::msg::Pose,
    pub relative_to_trajectory_id: i32,
}

impl Default for StartTrajectoryReq {
    fn default() -> Self {
        StartTrajectoryReq {
            configuration_directory: ::std::string::String::new(),
            configuration_basename: ::std::string::String::new(),
            use_initial_pose: false,
            initial_pose: crate::geometry_msgs::msg::Pose::default(),
            relative_to_trajectory_id: 0,
        }
    }
}

impl ros2_client::Message for StartTrajectoryReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartTrajectoryRes {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
    pub trajectory_id: i32,
}

impl Default for StartTrajectoryRes {
    fn default() -> Self {
        StartTrajectoryRes {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
            trajectory_id: 0,
        }
    }
}

impl ros2_client::Message for StartTrajectoryRes {}


pub struct StartTrajectory;
impl ros2_client::Service for StartTrajectory {
    type Request = StartTrajectoryReq;
    type Response = StartTrajectoryRes;

    fn request_type_name(&self) -> &str { "StartTrajectoryReq" }
    fn response_type_name(&self) -> &str { "StartTrajectoryRes" }
}
