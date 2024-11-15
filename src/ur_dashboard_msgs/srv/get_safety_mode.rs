use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSafetyModeReq {

}

impl Default for GetSafetyModeReq {
    fn default() -> Self {
        GetSafetyModeReq {

        }
    }
}

impl ros2_client::Message for GetSafetyModeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSafetyModeRes {
    pub safety_mode: crate::ur_dashboard_msgs::msg::SafetyMode,
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for GetSafetyModeRes {
    fn default() -> Self {
        GetSafetyModeRes {
            safety_mode: crate::ur_dashboard_msgs::msg::SafetyMode::default(),
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for GetSafetyModeRes {}


pub struct GetSafetyMode;
impl ros2_client::Service for GetSafetyMode {
    type Request = GetSafetyModeReq;
    type Response = GetSafetyModeRes;

    fn request_type_name(&self) -> &str { "GetSafetyModeReq" }
    fn response_type_name(&self) -> &str { "GetSafetyModeRes" }
}
