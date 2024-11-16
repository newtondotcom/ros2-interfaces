use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModeReq {

}

impl Default for GetModeReq {
    fn default() -> Self {
        GetModeReq {

        }
    }
}

impl ros2_client::Message for GetModeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModeRes {
    pub current_mode: ::std::string::String,
}

impl Default for GetModeRes {
    fn default() -> Self {
        GetModeRes {
            current_mode: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetModeRes {}


pub struct GetMode;
impl ros2_client::Service for GetMode {
    type Request = GetModeReq;
    type Response = GetModeRes;

    fn request_type_name(&self) -> &str { "GetModeReq" }
    fn response_type_name(&self) -> &str { "GetModeRes" }
}
