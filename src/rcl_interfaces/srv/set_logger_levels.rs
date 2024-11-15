use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLoggerLevelsReq {
    pub levels: Vec<crate::rcl_interfaces::msg::LoggerLevel>,
}

impl Default for SetLoggerLevelsReq {
    fn default() -> Self {
        SetLoggerLevelsReq {
            levels: Vec::new(),
        }
    }
}

impl ros2_client::Message for SetLoggerLevelsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLoggerLevelsRes {
    pub results: Vec<crate::rcl_interfaces::msg::SetLoggerLevelsResult>,
}

impl Default for SetLoggerLevelsRes {
    fn default() -> Self {
        SetLoggerLevelsRes {
            results: Vec::new(),
        }
    }
}

impl ros2_client::Message for SetLoggerLevelsRes {}


pub struct SetLoggerLevels;
impl ros2_client::Service for SetLoggerLevels {
    type Request = SetLoggerLevelsReq;
    type Response = SetLoggerLevelsRes;

    fn request_type_name(&self) -> &str { "SetLoggerLevelsReq" }
    fn response_type_name(&self) -> &str { "SetLoggerLevelsRes" }
}
