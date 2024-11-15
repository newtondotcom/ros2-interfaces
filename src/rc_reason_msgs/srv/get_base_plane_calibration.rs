use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBasePlaneCalibrationReq {
    pub pose_frame: ::std::string::String,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
}

impl Default for GetBasePlaneCalibrationReq {
    fn default() -> Self {
        GetBasePlaneCalibrationReq {
            pose_frame: ::std::string::String::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for GetBasePlaneCalibrationReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBasePlaneCalibrationRes {
    pub pose_frame: ::std::string::String,
    pub plane: crate::shape_msgs::msg::Plane,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for GetBasePlaneCalibrationRes {
    fn default() -> Self {
        GetBasePlaneCalibrationRes {
            pose_frame: ::std::string::String::new(),
            plane: crate::shape_msgs::msg::Plane::default(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for GetBasePlaneCalibrationRes {}


pub struct GetBasePlaneCalibration;
impl ros2_client::Service for GetBasePlaneCalibration {
    type Request = GetBasePlaneCalibrationReq;
    type Response = GetBasePlaneCalibrationRes;

    fn request_type_name(&self) -> &str { "GetBasePlaneCalibrationReq" }
    fn response_type_name(&self) -> &str { "GetBasePlaneCalibrationRes" }
}
