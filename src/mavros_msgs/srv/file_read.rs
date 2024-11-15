use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileReadReq {
    pub file_path: ::std::string::String,
    pub offset: u64,
    pub size: u64,
}

impl Default for FileReadReq {
    fn default() -> Self {
        FileReadReq {
            file_path: ::std::string::String::new(),
            offset: 0,
            size: 0,
        }
    }
}

impl ros2_client::Message for FileReadReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileReadRes {
    pub data: Vec<u8>,
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileReadRes {
    fn default() -> Self {
        FileReadRes {
            data: Vec::new(),
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileReadRes {}


pub struct FileRead;
impl ros2_client::Service for FileRead {
    type Request = FileReadReq;
    type Response = FileReadRes;

    fn request_type_name(&self) -> &str { "FileReadReq" }
    fn response_type_name(&self) -> &str { "FileReadRes" }
}
