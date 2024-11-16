use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRemoveDirReq {
    pub dir_path: ::std::string::String,
}

impl Default for FileRemoveDirReq {
    fn default() -> Self {
        FileRemoveDirReq {
            dir_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FileRemoveDirReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRemoveDirRes {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileRemoveDirRes {
    fn default() -> Self {
        FileRemoveDirRes {
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileRemoveDirRes {}


pub struct FileRemoveDir;
impl ros2_client::Service for FileRemoveDir {
    type Request = FileRemoveDirReq;
    type Response = FileRemoveDirRes;

    fn request_type_name(&self) -> &str { "FileRemoveDirReq" }
    fn response_type_name(&self) -> &str { "FileRemoveDirRes" }
}
