use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateScheduleReq {
    pub schedule: crate::rmf_scheduler_msgs::msg::Schedule,
}

impl Default for CreateScheduleReq {
    fn default() -> Self {
        CreateScheduleReq {
            schedule: crate::rmf_scheduler_msgs::msg::Schedule::default(),
        }
    }
}

impl ros2_client::Message for CreateScheduleReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateScheduleRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CreateScheduleRes {
    fn default() -> Self {
        CreateScheduleRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CreateScheduleRes {}


pub struct CreateSchedule;
impl ros2_client::Service for CreateSchedule {
    type Request = CreateScheduleReq;
    type Response = CreateScheduleRes;

    fn request_type_name(&self) -> &str { "CreateScheduleReq" }
    fn response_type_name(&self) -> &str { "CreateScheduleRes" }
}
