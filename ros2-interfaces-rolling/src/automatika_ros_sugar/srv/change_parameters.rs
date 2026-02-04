use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeParametersRequest {
    pub names: Vec<::std::string::String>,
    pub values: Vec<::std::string::String>,
    pub keep_alive: bool,
}

impl Default for ChangeParametersRequest {
    fn default() -> Self {
        ChangeParametersRequest {
            names: Vec::new(),
            values: Vec::new(),
            keep_alive: false,
        }
    }
}

impl ros2_client::Message for ChangeParametersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeParametersResponse {
    pub success: Vec<bool>,
    pub error_msg: Vec<::std::string::String>,
}

impl Default for ChangeParametersResponse {
    fn default() -> Self {
        ChangeParametersResponse {
            success: Vec::new(),
            error_msg: Vec::new(),
        }
    }
}

impl ros2_client::Message for ChangeParametersResponse {}


pub struct ChangeParameters;
impl ros2_client::Service for ChangeParameters {
    type Request = ChangeParametersRequest;
    type Response = ChangeParametersResponse;

    fn request_type_name(&self) -> &str { "ChangeParametersRequest" }
    fn response_type_name(&self) -> &str { "ChangeParametersResponse" }
}
