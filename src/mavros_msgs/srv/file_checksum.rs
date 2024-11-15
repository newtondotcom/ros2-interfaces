use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileChecksumReq {
    pub file_path: ::std::string::String,
}

impl Default for FileChecksumReq {
    fn default() -> Self {
        FileChecksumReq {
            file_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FileChecksumReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileChecksumRes {
    pub crc32: u32,
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileChecksumRes {
    fn default() -> Self {
        FileChecksumRes {
            crc32: 0,
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileChecksumRes {}


pub struct FileChecksum;
impl ros2_client::Service for FileChecksum {
    type Request = FileChecksumReq;
    type Response = FileChecksumRes;

    fn request_type_name(&self) -> &str { "FileChecksumReq" }
    fn response_type_name(&self) -> &str { "FileChecksumRes" }
}
