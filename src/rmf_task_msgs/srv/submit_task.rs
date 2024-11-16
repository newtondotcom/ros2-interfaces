use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitTaskReq {
    pub requester: ::std::string::String,
    pub description: crate::rmf_task_msgs::msg::TaskDescription,
}

impl Default for SubmitTaskReq {
    fn default() -> Self {
        SubmitTaskReq {
            requester: ::std::string::String::new(),
            description: crate::rmf_task_msgs::msg::TaskDescription::default(),
        }
    }
}

impl ros2_client::Message for SubmitTaskReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitTaskRes {
    pub success: bool,
    pub task_id: ::std::string::String,
    pub message: ::std::string::String,
}

impl Default for SubmitTaskRes {
    fn default() -> Self {
        SubmitTaskRes {
            success: false,
            task_id: ::std::string::String::new(),
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SubmitTaskRes {}


pub struct SubmitTask;
impl ros2_client::Service for SubmitTask {
    type Request = SubmitTaskReq;
    type Response = SubmitTaskRes;

    fn request_type_name(&self) -> &str { "SubmitTaskReq" }
    fn response_type_name(&self) -> &str { "SubmitTaskRes" }
}
