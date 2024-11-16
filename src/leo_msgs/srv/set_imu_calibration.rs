use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetImuCalibrationReq {
    pub gyro_bias_x: f64,
    pub gyro_bias_y: f64,
    pub gyro_bias_z: f64,
}

impl Default for SetImuCalibrationReq {
    fn default() -> Self {
        SetImuCalibrationReq {
            gyro_bias_x: 0.0,
            gyro_bias_y: 0.0,
            gyro_bias_z: 0.0,
        }
    }
}

impl ros2_client::Message for SetImuCalibrationReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetImuCalibrationRes {
    pub success: bool,
}

impl Default for SetImuCalibrationRes {
    fn default() -> Self {
        SetImuCalibrationRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SetImuCalibrationRes {}


pub struct SetImuCalibration;
impl ros2_client::Service for SetImuCalibration {
    type Request = SetImuCalibrationReq;
    type Response = SetImuCalibrationRes;

    fn request_type_name(&self) -> &str { "SetImuCalibrationReq" }
    fn response_type_name(&self) -> &str { "SetImuCalibrationRes" }
}
