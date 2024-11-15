use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemReq {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetProblemReq {
    fn default() -> Self {
        GetProblemReq {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetProblemReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemRes {
    pub success: bool,
    pub problem: ::std::string::String,
    pub error_info: ::std::string::String,
}

impl Default for GetProblemRes {
    fn default() -> Self {
        GetProblemRes {
            success: false,
            problem: ::std::string::String::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetProblemRes {}


pub struct GetProblem;
impl ros2_client::Service for GetProblem {
    type Request = GetProblemReq;
    type Response = GetProblemRes;

    fn request_type_name(&self) -> &str { "GetProblemReq" }
    fn response_type_name(&self) -> &str { "GetProblemRes" }
}
