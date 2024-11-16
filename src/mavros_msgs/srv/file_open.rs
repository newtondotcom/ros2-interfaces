use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileOpenReq {
    pub file_path: ::std::string::String,
    pub mode: u8,
}

impl FileOpenReq {
    pub const MODE_READ: u8 = 0;
    pub const MODE_WRITE: u8 = 1;
    pub const MODE_CREATE: u8 = 2;
}

impl Default for FileOpenReq {
    fn default() -> Self {
        FileOpenReq {
            file_path: ::std::string::String::new(),
            mode: 0,
        }
    }
}

impl ros2_client::Message for FileOpenReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileOpenRes {
    pub size: u32,
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileOpenRes {
    fn default() -> Self {
        FileOpenRes {
            size: 0,
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileOpenRes {}


pub struct FileOpen;
impl ros2_client::Service for FileOpen {
    type Request = FileOpenReq;
    type Response = FileOpenRes;

    fn request_type_name(&self) -> &str { "FileOpenReq" }
    fn response_type_name(&self) -> &str { "FileOpenRes" }
}
