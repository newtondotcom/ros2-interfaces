use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessFileReq {
    pub file_path: ::std::string::String,
    pub topic_name: ::std::string::String,
}

impl Default for ProcessFileReq {
    fn default() -> Self {
        ProcessFileReq {
            file_path: ::std::string::String::new(),
            topic_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ProcessFileReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessFileRes {
    pub success: bool,
}

impl Default for ProcessFileRes {
    fn default() -> Self {
        ProcessFileRes {
            success: false,
        }
    }
}

impl ros2_client::Message for ProcessFileRes {}


pub struct ProcessFile;
impl ros2_client::Service for ProcessFile {
    type Request = ProcessFileReq;
    type Response = ProcessFileRes;

    fn request_type_name(&self) -> &str { "ProcessFileReq" }
    fn response_type_name(&self) -> &str { "ProcessFileRes" }
}
