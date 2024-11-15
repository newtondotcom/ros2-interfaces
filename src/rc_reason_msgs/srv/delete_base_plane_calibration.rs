use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteBasePlaneCalibrationReq {

}

impl Default for DeleteBasePlaneCalibrationReq {
    fn default() -> Self {
        DeleteBasePlaneCalibrationReq {

        }
    }
}

impl ros2_client::Message for DeleteBasePlaneCalibrationReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteBasePlaneCalibrationRes {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteBasePlaneCalibrationRes {
    fn default() -> Self {
        DeleteBasePlaneCalibrationRes {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for DeleteBasePlaneCalibrationRes {}


pub struct DeleteBasePlaneCalibration;
impl ros2_client::Service for DeleteBasePlaneCalibration {
    type Request = DeleteBasePlaneCalibrationReq;
    type Response = DeleteBasePlaneCalibrationRes;

    fn request_type_name(&self) -> &str { "DeleteBasePlaneCalibrationReq" }
    fn response_type_name(&self) -> &str { "DeleteBasePlaneCalibrationRes" }
}
