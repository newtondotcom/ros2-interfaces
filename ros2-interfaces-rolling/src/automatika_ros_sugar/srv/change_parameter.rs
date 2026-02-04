use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeParameterRequest {
    pub name: ::std::string::String,
    pub value: ::std::string::String,
    pub keep_alive: bool,
}

impl Default for ChangeParameterRequest {
    fn default() -> Self {
        ChangeParameterRequest {
            name: ::std::string::String::new(),
            value: ::std::string::String::new(),
            keep_alive: false,
        }
    }
}

impl ros2_client::Message for ChangeParameterRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeParameterResponse {
    pub success: bool,
    pub error_msg: ::std::string::String,
}

impl Default for ChangeParameterResponse {
    fn default() -> Self {
        ChangeParameterResponse {
            success: false,
            error_msg: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ChangeParameterResponse {}


pub struct ChangeParameter;
impl ros2_client::Service for ChangeParameter {
    type Request = ChangeParameterRequest;
    type Response = ChangeParameterResponse;

    fn request_type_name(&self) -> &str { "ChangeParameterRequest" }
    fn response_type_name(&self) -> &str { "ChangeParameterResponse" }
}
