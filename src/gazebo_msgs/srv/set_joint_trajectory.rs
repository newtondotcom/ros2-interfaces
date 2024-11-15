use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetJointTrajectoryReq {
    pub model_name: ::std::string::String,
    pub joint_trajectory: crate::trajectory_msgs::msg::JointTrajectory,
    pub model_pose: crate::geometry_msgs::msg::Pose,
    pub set_model_pose: bool,
    pub disable_physics_updates: bool,
}

impl Default for SetJointTrajectoryReq {
    fn default() -> Self {
        SetJointTrajectoryReq {
            model_name: ::std::string::String::new(),
            joint_trajectory: crate::trajectory_msgs::msg::JointTrajectory::default(),
            model_pose: crate::geometry_msgs::msg::Pose::default(),
            set_model_pose: false,
            disable_physics_updates: false,
        }
    }
}

impl ros2_client::Message for SetJointTrajectoryReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetJointTrajectoryRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetJointTrajectoryRes {
    fn default() -> Self {
        SetJointTrajectoryRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetJointTrajectoryRes {}


pub struct SetJointTrajectory;
impl ros2_client::Service for SetJointTrajectory {
    type Request = SetJointTrajectoryReq;
    type Response = SetJointTrajectoryRes;

    fn request_type_name(&self) -> &str { "SetJointTrajectoryReq" }
    fn response_type_name(&self) -> &str { "SetJointTrajectoryRes" }
}
