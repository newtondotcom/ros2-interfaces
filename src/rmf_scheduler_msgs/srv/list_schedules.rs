use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListSchedulesReq {
    pub created_after: i64,
}

impl Default for ListSchedulesReq {
    fn default() -> Self {
        ListSchedulesReq {
            created_after: 0,
        }
    }
}

impl ros2_client::Message for ListSchedulesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListSchedulesRes {
    pub success: bool,
    pub message: ::std::string::String,
    pub schedules: Vec<crate::rmf_scheduler_msgs::msg::Schedule>,
}

impl Default for ListSchedulesRes {
    fn default() -> Self {
        ListSchedulesRes {
            success: false,
            message: ::std::string::String::new(),
            schedules: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListSchedulesRes {}


pub struct ListSchedules;
impl ros2_client::Service for ListSchedules {
    type Request = ListSchedulesReq;
    type Response = ListSchedulesRes;

    fn request_type_name(&self) -> &str { "ListSchedulesReq" }
    fn response_type_name(&self) -> &str { "ListSchedulesRes" }
}
