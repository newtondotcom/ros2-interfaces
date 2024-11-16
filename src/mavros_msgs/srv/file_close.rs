use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileCloseReq {
    pub file_path: ::std::string::String,
}

impl Default for FileCloseReq {
    fn default() -> Self {
        FileCloseReq {
            file_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FileCloseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileCloseRes {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileCloseRes {
    fn default() -> Self {
        FileCloseRes {
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileCloseRes {}


pub struct FileClose;
impl ros2_client::Service for FileClose {
    type Request = FileCloseReq;
    type Response = FileCloseRes;

    fn request_type_name(&self) -> &str { "FileCloseReq" }
    fn response_type_name(&self) -> &str { "FileCloseRes" }
}
