use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListScheduleStatesReq {
    pub modified_after: i64,
}

impl Default for ListScheduleStatesReq {
    fn default() -> Self {
        ListScheduleStatesReq {
            modified_after: 0,
        }
    }
}

impl ros2_client::Message for ListScheduleStatesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListScheduleStatesRes {
    pub success: bool,
    pub message: ::std::string::String,
    pub schedules: Vec<crate::rmf_scheduler_msgs::msg::ScheduleState>,
}

impl Default for ListScheduleStatesRes {
    fn default() -> Self {
        ListScheduleStatesRes {
            success: false,
            message: ::std::string::String::new(),
            schedules: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListScheduleStatesRes {}


pub struct ListScheduleStates;
impl ros2_client::Service for ListScheduleStates {
    type Request = ListScheduleStatesReq;
    type Response = ListScheduleStatesRes;

    fn request_type_name(&self) -> &str { "ListScheduleStatesReq" }
    fn response_type_name(&self) -> &str { "ListScheduleStatesRes" }
}
