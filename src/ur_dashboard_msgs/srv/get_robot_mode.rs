use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotModeReq {

}

impl Default for GetRobotModeReq {
    fn default() -> Self {
        GetRobotModeReq {

        }
    }
}

impl ros2_client::Message for GetRobotModeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotModeRes {
    pub robot_mode: crate::ur_dashboard_msgs::msg::RobotMode,
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for GetRobotModeRes {
    fn default() -> Self {
        GetRobotModeRes {
            robot_mode: crate::ur_dashboard_msgs::msg::RobotMode::default(),
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for GetRobotModeRes {}


pub struct GetRobotMode;
impl ros2_client::Service for GetRobotMode {
    type Request = GetRobotModeReq;
    type Response = GetRobotModeRes;

    fn request_type_name(&self) -> &str { "GetRobotModeReq" }
    fn response_type_name(&self) -> &str { "GetRobotModeRes" }
}
