use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviveTaskReq {
    pub requester: ::std::string::String,
    pub task_id: ::std::string::String,
}

impl Default for ReviveTaskReq {
    fn default() -> Self {
        ReviveTaskReq {
            requester: ::std::string::String::new(),
            task_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ReviveTaskReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviveTaskRes {
    pub success: bool,
}

impl Default for ReviveTaskRes {
    fn default() -> Self {
        ReviveTaskRes {
            success: false,
        }
    }
}

impl ros2_client::Message for ReviveTaskRes {}


pub struct ReviveTask;
impl ros2_client::Service for ReviveTask {
    type Request = ReviveTaskReq;
    type Response = ReviveTaskRes;

    fn request_type_name(&self) -> &str { "ReviveTaskReq" }
    fn response_type_name(&self) -> &str { "ReviveTaskRes" }
}
