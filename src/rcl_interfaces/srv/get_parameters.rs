use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParametersReq {
    pub names: Vec<::std::string::String>,
}

impl Default for GetParametersReq {
    fn default() -> Self {
        GetParametersReq {
            names: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetParametersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParametersRes {
    pub values: Vec<crate::rcl_interfaces::msg::ParameterValue>,
}

impl Default for GetParametersRes {
    fn default() -> Self {
        GetParametersRes {
            values: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetParametersRes {}


pub struct GetParameters;
impl ros2_client::Service for GetParameters {
    type Request = GetParametersReq;
    type Response = GetParametersRes;

    fn request_type_name(&self) -> &str { "GetParametersReq" }
    fn response_type_name(&self) -> &str { "GetParametersRes" }
}
