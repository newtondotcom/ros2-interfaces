use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileMakeDirReq {
    pub dir_path: ::std::string::String,
}

impl Default for FileMakeDirReq {
    fn default() -> Self {
        FileMakeDirReq {
            dir_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FileMakeDirReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileMakeDirRes {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileMakeDirRes {
    fn default() -> Self {
        FileMakeDirRes {
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileMakeDirRes {}


pub struct FileMakeDir;
impl ros2_client::Service for FileMakeDir {
    type Request = FileMakeDirReq;
    type Response = FileMakeDirRes;

    fn request_type_name(&self) -> &str { "FileMakeDirReq" }
    fn response_type_name(&self) -> &str { "FileMakeDirRes" }
}
