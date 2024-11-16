use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParameterTypesReq {
    pub names: Vec<::std::string::String>,
}

impl Default for GetParameterTypesReq {
    fn default() -> Self {
        GetParameterTypesReq {
            names: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetParameterTypesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParameterTypesRes {
    pub types: Vec<u8>,
}

impl Default for GetParameterTypesRes {
    fn default() -> Self {
        GetParameterTypesRes {
            types: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetParameterTypesRes {}


pub struct GetParameterTypes;
impl ros2_client::Service for GetParameterTypes {
    type Request = GetParameterTypesReq;
    type Response = GetParameterTypesRes;

    fn request_type_name(&self) -> &str { "GetParameterTypesReq" }
    fn response_type_name(&self) -> &str { "GetParameterTypesRes" }
}
