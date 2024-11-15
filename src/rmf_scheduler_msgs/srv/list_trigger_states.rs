use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListTriggerStatesReq {
    pub modified_after: i64,
}

impl Default for ListTriggerStatesReq {
    fn default() -> Self {
        ListTriggerStatesReq {
            modified_after: 0,
        }
    }
}

impl ros2_client::Message for ListTriggerStatesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListTriggerStatesRes {
    pub success: bool,
    pub message: ::std::string::String,
    pub triggers: Vec<crate::rmf_scheduler_msgs::msg::TriggerState>,
}

impl Default for ListTriggerStatesRes {
    fn default() -> Self {
        ListTriggerStatesRes {
            success: false,
            message: ::std::string::String::new(),
            triggers: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListTriggerStatesRes {}


pub struct ListTriggerStates;
impl ros2_client::Service for ListTriggerStates {
    type Request = ListTriggerStatesReq;
    type Response = ListTriggerStatesRes;

    fn request_type_name(&self) -> &str { "ListTriggerStatesReq" }
    fn response_type_name(&self) -> &str { "ListTriggerStatesRes" }
}
