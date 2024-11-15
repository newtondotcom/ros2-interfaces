use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileWriteReq {
    pub file_path: ::std::string::String,
    pub offset: u64,
    pub data: Vec<u8>,
}

impl Default for FileWriteReq {
    fn default() -> Self {
        FileWriteReq {
            file_path: ::std::string::String::new(),
            offset: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for FileWriteReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileWriteRes {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileWriteRes {
    fn default() -> Self {
        FileWriteRes {
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileWriteRes {}


pub struct FileWrite;
impl ros2_client::Service for FileWrite {
    type Request = FileWriteReq;
    type Response = FileWriteRes;

    fn request_type_name(&self) -> &str { "FileWriteReq" }
    fn response_type_name(&self) -> &str { "FileWriteRes" }
}
