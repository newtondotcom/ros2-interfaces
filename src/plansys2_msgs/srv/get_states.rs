use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStatesReq {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetStatesReq {
    fn default() -> Self {
        GetStatesReq {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetStatesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStatesRes {
    pub success: bool,
    pub states: Vec<crate::plansys2_msgs::msg::Node>,
    pub error_info: ::std::string::String,
}

impl Default for GetStatesRes {
    fn default() -> Self {
        GetStatesRes {
            success: false,
            states: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetStatesRes {}


pub struct GetStates;
impl ros2_client::Service for GetStates {
    type Request = GetStatesReq;
    type Response = GetStatesRes;

    fn request_type_name(&self) -> &str { "GetStatesReq" }
    fn response_type_name(&self) -> &str { "GetStatesRes" }
}
