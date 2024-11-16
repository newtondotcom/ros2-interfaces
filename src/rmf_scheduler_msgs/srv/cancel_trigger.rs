use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelTriggerReq {
    pub name: ::std::string::String,
}

impl Default for CancelTriggerReq {
    fn default() -> Self {
        CancelTriggerReq {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CancelTriggerReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelTriggerRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CancelTriggerRes {
    fn default() -> Self {
        CancelTriggerRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CancelTriggerRes {}


pub struct CancelTrigger;
impl ros2_client::Service for CancelTrigger {
    type Request = CancelTriggerReq;
    type Response = CancelTriggerRes;

    fn request_type_name(&self) -> &str { "CancelTriggerReq" }
    fn response_type_name(&self) -> &str { "CancelTriggerRes" }
}
