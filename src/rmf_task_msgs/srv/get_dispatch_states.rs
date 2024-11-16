use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDispatchStatesReq {
    pub task_ids: Vec<::std::string::String>,
}

impl Default for GetDispatchStatesReq {
    fn default() -> Self {
        GetDispatchStatesReq {
            task_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetDispatchStatesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDispatchStatesRes {
    pub success: bool,
    pub states: crate::rmf_task_msgs::msg::DispatchStates,
}

impl Default for GetDispatchStatesRes {
    fn default() -> Self {
        GetDispatchStatesRes {
            success: false,
            states: crate::rmf_task_msgs::msg::DispatchStates::default(),
        }
    }
}

impl ros2_client::Message for GetDispatchStatesRes {}


pub struct GetDispatchStates;
impl ros2_client::Service for GetDispatchStates {
    type Request = GetDispatchStatesReq;
    type Response = GetDispatchStatesRes;

    fn request_type_name(&self) -> &str { "GetDispatchStatesReq" }
    fn response_type_name(&self) -> &str { "GetDispatchStatesRes" }
}
