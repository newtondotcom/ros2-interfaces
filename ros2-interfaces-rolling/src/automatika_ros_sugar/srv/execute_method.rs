use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteMethodRequest {
    pub name: ::std::string::String,
    pub kwargs_json: ::std::string::String,
}

impl Default for ExecuteMethodRequest {
    fn default() -> Self {
        ExecuteMethodRequest {
            name: ::std::string::String::new(),
            kwargs_json: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ExecuteMethodRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteMethodResponse {
    pub success: bool,
    pub error_msg: ::std::string::String,
}

impl Default for ExecuteMethodResponse {
    fn default() -> Self {
        ExecuteMethodResponse {
            success: false,
            error_msg: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ExecuteMethodResponse {}


pub struct ExecuteMethod;
impl ros2_client::Service for ExecuteMethod {
    type Request = ExecuteMethodRequest;
    type Response = ExecuteMethodResponse;

    fn request_type_name(&self) -> &str { "ExecuteMethodRequest" }
    fn response_type_name(&self) -> &str { "ExecuteMethodResponse" }
}
