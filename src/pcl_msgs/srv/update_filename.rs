use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateFilenameReq {
    pub filename: ::std::string::String,
}

impl Default for UpdateFilenameReq {
    fn default() -> Self {
        UpdateFilenameReq {
            filename: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UpdateFilenameReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateFilenameRes {
    pub success: bool,
}

impl Default for UpdateFilenameRes {
    fn default() -> Self {
        UpdateFilenameRes {
            success: false,
        }
    }
}

impl ros2_client::Message for UpdateFilenameRes {}


pub struct UpdateFilename;
impl ros2_client::Service for UpdateFilename {
    type Request = UpdateFilenameReq;
    type Response = UpdateFilenameRes;

    fn request_type_name(&self) -> &str { "UpdateFilenameReq" }
    fn response_type_name(&self) -> &str { "UpdateFilenameRes" }
}
