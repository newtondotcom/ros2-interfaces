use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandEyeCalibrationTriggerReq {

}

impl Default for HandEyeCalibrationTriggerReq {
    fn default() -> Self {
        HandEyeCalibrationTriggerReq {

        }
    }
}

impl ros2_client::Message for HandEyeCalibrationTriggerReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandEyeCalibrationTriggerRes {
    pub success: bool,
    pub status: i32,
    pub message: ::std::string::String,
}

impl Default for HandEyeCalibrationTriggerRes {
    fn default() -> Self {
        HandEyeCalibrationTriggerRes {
            success: false,
            status: 0,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for HandEyeCalibrationTriggerRes {}


pub struct HandEyeCalibrationTrigger;
impl ros2_client::Service for HandEyeCalibrationTrigger {
    type Request = HandEyeCalibrationTriggerReq;
    type Response = HandEyeCalibrationTriggerRes;

    fn request_type_name(&self) -> &str { "HandEyeCalibrationTriggerReq" }
    fn response_type_name(&self) -> &str { "HandEyeCalibrationTriggerRes" }
}
