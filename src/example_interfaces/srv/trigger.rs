use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerReq {

}

impl Default for TriggerReq {
    fn default() -> Self {
        TriggerReq {

        }
    }
}

impl ros2_client::Message for TriggerReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for TriggerRes {
    fn default() -> Self {
        TriggerRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for TriggerRes {}


pub struct Trigger;
impl ros2_client::Service for Trigger {
    type Request = TriggerReq;
    type Response = TriggerRes;

    fn request_type_name(&self) -> &str { "TriggerReq" }
    fn response_type_name(&self) -> &str { "TriggerRes" }
}
