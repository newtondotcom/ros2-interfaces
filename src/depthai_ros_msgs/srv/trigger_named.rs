use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerNamedReq {
    pub name: ::std::string::String,
}

impl Default for TriggerNamedReq {
    fn default() -> Self {
        TriggerNamedReq {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for TriggerNamedReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerNamedRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for TriggerNamedRes {
    fn default() -> Self {
        TriggerNamedRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for TriggerNamedRes {}


pub struct TriggerNamed;
impl ros2_client::Service for TriggerNamed {
    type Request = TriggerNamedReq;
    type Response = TriggerNamedRes;

    fn request_type_name(&self) -> &str { "TriggerNamedReq" }
    fn response_type_name(&self) -> &str { "TriggerNamedRes" }
}
