use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeModeReq {
    pub mode_name: ::std::string::String,
}

impl Default for ChangeModeReq {
    fn default() -> Self {
        ChangeModeReq {
            mode_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ChangeModeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeModeRes {
    pub success: bool,
}

impl Default for ChangeModeRes {
    fn default() -> Self {
        ChangeModeRes {
            success: false,
        }
    }
}

impl ros2_client::Message for ChangeModeRes {}


pub struct ChangeMode;
impl ros2_client::Service for ChangeMode {
    type Request = ChangeModeReq;
    type Response = ChangeModeRes;

    fn request_type_name(&self) -> &str { "ChangeModeReq" }
    fn response_type_name(&self) -> &str { "ChangeModeRes" }
}
