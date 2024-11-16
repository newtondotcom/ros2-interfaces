use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRenameReq {
    pub old_path: ::std::string::String,
    pub new_path: ::std::string::String,
}

impl Default for FileRenameReq {
    fn default() -> Self {
        FileRenameReq {
            old_path: ::std::string::String::new(),
            new_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FileRenameReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRenameRes {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileRenameRes {
    fn default() -> Self {
        FileRenameRes {
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileRenameRes {}


pub struct FileRename;
impl ros2_client::Service for FileRename {
    type Request = FileRenameReq;
    type Response = FileRenameRes;

    fn request_type_name(&self) -> &str { "FileRenameReq" }
    fn response_type_name(&self) -> &str { "FileRenameRes" }
}
