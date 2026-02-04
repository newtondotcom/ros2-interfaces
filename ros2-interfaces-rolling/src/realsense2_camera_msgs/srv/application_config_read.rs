use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationConfigReadRequest {

}

impl Default for ApplicationConfigReadRequest {
    fn default() -> Self {
        ApplicationConfigReadRequest {

        }
    }
}

impl ros2_client::Message for ApplicationConfigReadRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationConfigReadResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
    pub application_config: ::std::string::String,
}

impl Default for ApplicationConfigReadResponse {
    fn default() -> Self {
        ApplicationConfigReadResponse {
            success: false,
            error_message: ::std::string::String::new(),
            application_config: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ApplicationConfigReadResponse {}


pub struct ApplicationConfigRead;
impl ros2_client::Service for ApplicationConfigRead {
    type Request = ApplicationConfigReadRequest;
    type Response = ApplicationConfigReadResponse;

    fn request_type_name(&self) -> &str { "ApplicationConfigReadRequest" }
    fn response_type_name(&self) -> &str { "ApplicationConfigReadResponse" }
}
