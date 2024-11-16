use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddToLogReq {
    pub message: ::std::string::String,
}

impl Default for AddToLogReq {
    fn default() -> Self {
        AddToLogReq {
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AddToLogReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddToLogRes {
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for AddToLogRes {
    fn default() -> Self {
        AddToLogRes {
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for AddToLogRes {}


pub struct AddToLog;
impl ros2_client::Service for AddToLog {
    type Request = AddToLogReq;
    type Response = AddToLogRes;

    fn request_type_name(&self) -> &str { "AddToLogReq" }
    fn response_type_name(&self) -> &str { "AddToLogRes" }
}
