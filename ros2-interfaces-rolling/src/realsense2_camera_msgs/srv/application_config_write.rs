use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationConfigWriteRequest {
    pub application_config: ::std::string::String,
}

impl Default for ApplicationConfigWriteRequest {
    fn default() -> Self {
        ApplicationConfigWriteRequest {
            application_config: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ApplicationConfigWriteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationConfigWriteResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
}

impl Default for ApplicationConfigWriteResponse {
    fn default() -> Self {
        ApplicationConfigWriteResponse {
            success: false,
            error_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ApplicationConfigWriteResponse {}


pub struct ApplicationConfigWrite;
impl ros2_client::Service for ApplicationConfigWrite {
    type Request = ApplicationConfigWriteRequest;
    type Response = ApplicationConfigWriteResponse;

    fn request_type_name(&self) -> &str { "ApplicationConfigWriteRequest" }
    fn response_type_name(&self) -> &str { "ApplicationConfigWriteResponse" }
}
