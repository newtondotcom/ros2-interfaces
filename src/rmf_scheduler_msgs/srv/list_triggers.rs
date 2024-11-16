use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListTriggersReq {
    pub created_after: i64,
}

impl Default for ListTriggersReq {
    fn default() -> Self {
        ListTriggersReq {
            created_after: 0,
        }
    }
}

impl ros2_client::Message for ListTriggersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListTriggersRes {
    pub success: bool,
    pub message: ::std::string::String,
    pub triggers: Vec<crate::rmf_scheduler_msgs::msg::Trigger>,
}

impl Default for ListTriggersRes {
    fn default() -> Self {
        ListTriggersRes {
            success: false,
            message: ::std::string::String::new(),
            triggers: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListTriggersRes {}


pub struct ListTriggers;
impl ros2_client::Service for ListTriggers {
    type Request = ListTriggersReq;
    type Response = ListTriggersRes;

    fn request_type_name(&self) -> &str { "ListTriggersReq" }
    fn response_type_name(&self) -> &str { "ListTriggersRes" }
}
