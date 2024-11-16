use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelScheduleReq {
    pub name: ::std::string::String,
    pub finished: bool,
}

impl Default for CancelScheduleReq {
    fn default() -> Self {
        CancelScheduleReq {
            name: ::std::string::String::new(),
            finished: false,
        }
    }
}

impl ros2_client::Message for CancelScheduleReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelScheduleRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CancelScheduleRes {
    fn default() -> Self {
        CancelScheduleRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CancelScheduleRes {}


pub struct CancelSchedule;
impl ros2_client::Service for CancelSchedule {
    type Request = CancelScheduleReq;
    type Response = CancelScheduleRes;

    fn request_type_name(&self) -> &str { "CancelScheduleReq" }
    fn response_type_name(&self) -> &str { "CancelScheduleRes" }
}
