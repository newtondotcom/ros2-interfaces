use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadReq {
    pub filename: ::std::string::String,
}

impl Default for LoadReq {
    fn default() -> Self {
        LoadReq {
            filename: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LoadReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadRes {
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for LoadRes {
    fn default() -> Self {
        LoadRes {
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for LoadRes {}


pub struct Load;
impl ros2_client::Service for Load {
    type Request = LoadReq;
    type Response = LoadRes;

    fn request_type_name(&self) -> &str { "LoadReq" }
    fn response_type_name(&self) -> &str { "LoadRes" }
}
