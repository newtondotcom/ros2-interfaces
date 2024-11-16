use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRemoveReq {
    pub file_path: ::std::string::String,
}

impl Default for FileRemoveReq {
    fn default() -> Self {
        FileRemoveReq {
            file_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FileRemoveReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRemoveRes {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileRemoveRes {
    fn default() -> Self {
        FileRemoveRes {
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileRemoveRes {}


pub struct FileRemove;
impl ros2_client::Service for FileRemove {
    type Request = FileRemoveReq;
    type Response = FileRemoveRes;

    fn request_type_name(&self) -> &str { "FileRemoveReq" }
    fn response_type_name(&self) -> &str { "FileRemoveRes" }
}
