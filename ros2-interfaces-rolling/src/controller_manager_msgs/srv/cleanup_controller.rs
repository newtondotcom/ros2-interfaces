use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CleanupControllerRequest {
    pub name: ::std::string::String,
}

impl Default for CleanupControllerRequest {
    fn default() -> Self {
        CleanupControllerRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CleanupControllerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CleanupControllerResponse {
    pub ok: bool,
}

impl Default for CleanupControllerResponse {
    fn default() -> Self {
        CleanupControllerResponse {
            ok: false,
        }
    }
}

impl ros2_client::Message for CleanupControllerResponse {}


pub struct CleanupController;
impl ros2_client::Service for CleanupController {
    type Request = CleanupControllerRequest;
    type Response = CleanupControllerResponse;

    fn request_type_name(&self) -> &str { "CleanupControllerRequest" }
    fn response_type_name(&self) -> &str { "CleanupControllerResponse" }
}
