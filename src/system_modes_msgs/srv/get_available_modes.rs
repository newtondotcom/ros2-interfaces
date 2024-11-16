use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableModesReq {

}

impl Default for GetAvailableModesReq {
    fn default() -> Self {
        GetAvailableModesReq {

        }
    }
}

impl ros2_client::Message for GetAvailableModesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableModesRes {
    pub available_modes: Vec<::std::string::String>,
}

impl Default for GetAvailableModesRes {
    fn default() -> Self {
        GetAvailableModesRes {
            available_modes: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetAvailableModesRes {}


pub struct GetAvailableModes;
impl ros2_client::Service for GetAvailableModes {
    type Request = GetAvailableModesReq;
    type Response = GetAvailableModesRes;

    fn request_type_name(&self) -> &str { "GetAvailableModesReq" }
    fn response_type_name(&self) -> &str { "GetAvailableModesRes" }
}
