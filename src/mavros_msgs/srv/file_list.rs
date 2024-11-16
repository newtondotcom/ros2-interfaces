use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileListReq {
    pub dir_path: ::std::string::String,
}

impl Default for FileListReq {
    fn default() -> Self {
        FileListReq {
            dir_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FileListReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileListRes {
    pub list: Vec<crate::mavros_msgs::msg::FileEntry>,
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileListRes {
    fn default() -> Self {
        FileListRes {
            list: Vec::new(),
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileListRes {}


pub struct FileList;
impl ros2_client::Service for FileList {
    type Request = FileListReq;
    type Response = FileListRes;

    fn request_type_name(&self) -> &str { "FileListReq" }
    fn response_type_name(&self) -> &str { "FileListRes" }
}
