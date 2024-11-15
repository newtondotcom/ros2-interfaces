use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoggerLevelsReq {
    pub names: Vec<::std::string::String>,
}

impl Default for GetLoggerLevelsReq {
    fn default() -> Self {
        GetLoggerLevelsReq {
            names: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetLoggerLevelsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoggerLevelsRes {
    pub levels: Vec<crate::rcl_interfaces::msg::LoggerLevel>,
}

impl Default for GetLoggerLevelsRes {
    fn default() -> Self {
        GetLoggerLevelsRes {
            levels: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetLoggerLevelsRes {}


pub struct GetLoggerLevels;
impl ros2_client::Service for GetLoggerLevels {
    type Request = GetLoggerLevelsReq;
    type Response = GetLoggerLevelsRes;

    fn request_type_name(&self) -> &str { "GetLoggerLevelsReq" }
    fn response_type_name(&self) -> &str { "GetLoggerLevelsRes" }
}
