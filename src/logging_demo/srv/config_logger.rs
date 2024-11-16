use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigLoggerReq {
    pub logger_name: ::std::string::String,
    pub level: ::std::string::String,
}

impl Default for ConfigLoggerReq {
    fn default() -> Self {
        ConfigLoggerReq {
            logger_name: ::std::string::String::new(),
            level: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ConfigLoggerReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigLoggerRes {
    pub success: bool,
}

impl Default for ConfigLoggerRes {
    fn default() -> Self {
        ConfigLoggerRes {
            success: false,
        }
    }
}

impl ros2_client::Message for ConfigLoggerRes {}


pub struct ConfigLogger;
impl ros2_client::Service for ConfigLogger {
    type Request = ConfigLoggerReq;
    type Response = ConfigLoggerRes;

    fn request_type_name(&self) -> &str { "ConfigLoggerReq" }
    fn response_type_name(&self) -> &str { "ConfigLoggerRes" }
}
