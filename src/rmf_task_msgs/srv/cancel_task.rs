use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelTaskReq {
    pub requester: ::std::string::String,
    pub task_id: ::std::string::String,
}

impl Default for CancelTaskReq {
    fn default() -> Self {
        CancelTaskReq {
            requester: ::std::string::String::new(),
            task_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CancelTaskReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelTaskRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CancelTaskRes {
    fn default() -> Self {
        CancelTaskRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CancelTaskRes {}


pub struct CancelTask;
impl ros2_client::Service for CancelTask {
    type Request = CancelTaskReq;
    type Response = CancelTaskRes;

    fn request_type_name(&self) -> &str { "CancelTaskReq" }
    fn response_type_name(&self) -> &str { "CancelTaskRes" }
}
