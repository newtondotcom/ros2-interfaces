use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibConfigWriteRequest {
    pub calib_config: ::std::string::String,
}

impl Default for CalibConfigWriteRequest {
    fn default() -> Self {
        CalibConfigWriteRequest {
            calib_config: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CalibConfigWriteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibConfigWriteResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
}

impl Default for CalibConfigWriteResponse {
    fn default() -> Self {
        CalibConfigWriteResponse {
            success: false,
            error_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CalibConfigWriteResponse {}


pub struct CalibConfigWrite;
impl ros2_client::Service for CalibConfigWrite {
    type Request = CalibConfigWriteRequest;
    type Response = CalibConfigWriteResponse;

    fn request_type_name(&self) -> &str { "CalibConfigWriteRequest" }
    fn response_type_name(&self) -> &str { "CalibConfigWriteResponse" }
}
