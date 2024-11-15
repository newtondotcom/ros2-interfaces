use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileTruncateReq {
    pub file_path: ::std::string::String,
    pub length: u64,
}

impl Default for FileTruncateReq {
    fn default() -> Self {
        FileTruncateReq {
            file_path: ::std::string::String::new(),
            length: 0,
        }
    }
}

impl ros2_client::Message for FileTruncateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileTruncateRes {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileTruncateRes {
    fn default() -> Self {
        FileTruncateRes {
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileTruncateRes {}


pub struct FileTruncate;
impl ros2_client::Service for FileTruncate {
    type Request = FileTruncateReq;
    type Response = FileTruncateRes;

    fn request_type_name(&self) -> &str { "FileTruncateReq" }
    fn response_type_name(&self) -> &str { "FileTruncateRes" }
}
