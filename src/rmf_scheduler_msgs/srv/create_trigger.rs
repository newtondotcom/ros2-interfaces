use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTriggerReq {
    pub trigger: crate::rmf_scheduler_msgs::msg::Trigger,
}

impl Default for CreateTriggerReq {
    fn default() -> Self {
        CreateTriggerReq {
            trigger: crate::rmf_scheduler_msgs::msg::Trigger::default(),
        }
    }
}

impl ros2_client::Message for CreateTriggerReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTriggerRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CreateTriggerRes {
    fn default() -> Self {
        CreateTriggerRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CreateTriggerRes {}


pub struct CreateTrigger;
impl ros2_client::Service for CreateTrigger {
    type Request = CreateTriggerReq;
    type Response = CreateTriggerRes;

    fn request_type_name(&self) -> &str { "CreateTriggerReq" }
    fn response_type_name(&self) -> &str { "CreateTriggerRes" }
}
