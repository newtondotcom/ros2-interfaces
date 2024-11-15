use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotPowerReq {

}

impl Default for RobotPowerReq {
    fn default() -> Self {
        RobotPowerReq {

        }
    }
}

impl ros2_client::Message for RobotPowerReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotPowerRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for RobotPowerRes {
    fn default() -> Self {
        RobotPowerRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RobotPowerRes {}


pub struct RobotPower;
impl ros2_client::Service for RobotPower {
    type Request = RobotPowerReq;
    type Response = RobotPowerRes;

    fn request_type_name(&self) -> &str { "RobotPowerReq" }
    fn response_type_name(&self) -> &str { "RobotPowerRes" }
}
