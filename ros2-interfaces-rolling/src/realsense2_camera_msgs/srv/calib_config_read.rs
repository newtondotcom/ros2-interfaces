use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibConfigReadRequest {

}

impl Default for CalibConfigReadRequest {
    fn default() -> Self {
        CalibConfigReadRequest {

        }
    }
}

impl ros2_client::Message for CalibConfigReadRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibConfigReadResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
    pub calib_config: ::std::string::String,
}

impl Default for CalibConfigReadResponse {
    fn default() -> Self {
        CalibConfigReadResponse {
            success: false,
            error_message: ::std::string::String::new(),
            calib_config: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CalibConfigReadResponse {}


pub struct CalibConfigRead;
impl ros2_client::Service for CalibConfigRead {
    type Request = CalibConfigReadRequest;
    type Response = CalibConfigReadResponse;

    fn request_type_name(&self) -> &str { "CalibConfigReadRequest" }
    fn response_type_name(&self) -> &str { "CalibConfigReadResponse" }
}
