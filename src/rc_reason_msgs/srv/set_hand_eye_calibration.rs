use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHandEyeCalibrationReq {
    pub pose: crate::geometry_msgs::msg::Pose,
    pub robot_mounted: bool,
}

impl Default for SetHandEyeCalibrationReq {
    fn default() -> Self {
        SetHandEyeCalibrationReq {
            pose: crate::geometry_msgs::msg::Pose::default(),
            robot_mounted: false,
        }
    }
}

impl ros2_client::Message for SetHandEyeCalibrationReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHandEyeCalibrationRes {
    pub success: bool,
    pub status: i32,
    pub message: ::std::string::String,
}

impl Default for SetHandEyeCalibrationRes {
    fn default() -> Self {
        SetHandEyeCalibrationRes {
            success: false,
            status: 0,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetHandEyeCalibrationRes {}


pub struct SetHandEyeCalibration;
impl ros2_client::Service for SetHandEyeCalibration {
    type Request = SetHandEyeCalibrationReq;
    type Response = SetHandEyeCalibrationRes;

    fn request_type_name(&self) -> &str { "SetHandEyeCalibrationReq" }
    fn response_type_name(&self) -> &str { "SetHandEyeCalibrationRes" }
}
