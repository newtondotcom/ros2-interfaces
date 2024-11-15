use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParamNamesReq {

}

impl Default for GetParamNamesReq {
    fn default() -> Self {
        GetParamNamesReq {

        }
    }
}

impl ros2_client::Message for GetParamNamesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParamNamesRes {
    pub names: Vec<::std::string::String>,
}

impl Default for GetParamNamesRes {
    fn default() -> Self {
        GetParamNamesRes {
            names: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetParamNamesRes {}


pub struct GetParamNames;
impl ros2_client::Service for GetParamNames {
    type Request = GetParamNamesReq;
    type Response = GetParamNamesRes;

    fn request_type_name(&self) -> &str { "GetParamNamesReq" }
    fn response_type_name(&self) -> &str { "GetParamNamesRes" }
}
