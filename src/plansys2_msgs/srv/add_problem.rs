use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddProblemReq {
    pub problem: ::std::string::String,
}

impl Default for AddProblemReq {
    fn default() -> Self {
        AddProblemReq {
            problem: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AddProblemReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddProblemRes {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for AddProblemRes {
    fn default() -> Self {
        AddProblemRes {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AddProblemRes {}


pub struct AddProblem;
impl ros2_client::Service for AddProblem {
    type Request = AddProblemReq;
    type Response = AddProblemRes;

    fn request_type_name(&self) -> &str { "AddProblemReq" }
    fn response_type_name(&self) -> &str { "AddProblemRes" }
}
