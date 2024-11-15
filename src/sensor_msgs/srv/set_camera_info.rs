use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCameraInfoReq {
    pub camera_info: crate::sensor_msgs::msg::CameraInfo,
}

impl Default for SetCameraInfoReq {
    fn default() -> Self {
        SetCameraInfoReq {
            camera_info: crate::sensor_msgs::msg::CameraInfo::default(),
        }
    }
}

impl ros2_client::Message for SetCameraInfoReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCameraInfoRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetCameraInfoRes {
    fn default() -> Self {
        SetCameraInfoRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetCameraInfoRes {}


pub struct SetCameraInfo;
impl ros2_client::Service for SetCameraInfo {
    type Request = SetCameraInfoReq;
    type Response = SetCameraInfoRes;

    fn request_type_name(&self) -> &str { "SetCameraInfoReq" }
    fn response_type_name(&self) -> &str { "SetCameraInfoRes" }
}
