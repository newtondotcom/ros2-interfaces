use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHandEyeCalibrationPoseReq {
    pub slot: i32,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for SetHandEyeCalibrationPoseReq {
    fn default() -> Self {
        SetHandEyeCalibrationPoseReq {
            slot: 0,
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for SetHandEyeCalibrationPoseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHandEyeCalibrationPoseRes {
    pub success: bool,
    pub status: i32,
    pub message: ::std::string::String,
}

impl Default for SetHandEyeCalibrationPoseRes {
    fn default() -> Self {
        SetHandEyeCalibrationPoseRes {
            success: false,
            status: 0,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetHandEyeCalibrationPoseRes {}


pub struct SetHandEyeCalibrationPose;
impl ros2_client::Service for SetHandEyeCalibrationPose {
    type Request = SetHandEyeCalibrationPoseReq;
    type Response = SetHandEyeCalibrationPoseRes;

    fn request_type_name(&self) -> &str { "SetHandEyeCalibrationPoseReq" }
    fn response_type_name(&self) -> &str { "SetHandEyeCalibrationPoseRes" }
}
