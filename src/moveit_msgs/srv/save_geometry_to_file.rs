use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveGeometryToFileReq {
    pub file_path_and_name: ::std::string::String,
}

impl Default for SaveGeometryToFileReq {
    fn default() -> Self {
        SaveGeometryToFileReq {
            file_path_and_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveGeometryToFileReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveGeometryToFileRes {
    pub success: bool,
}

impl Default for SaveGeometryToFileRes {
    fn default() -> Self {
        SaveGeometryToFileRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SaveGeometryToFileRes {}


pub struct SaveGeometryToFile;
impl ros2_client::Service for SaveGeometryToFile {
    type Request = SaveGeometryToFileReq;
    type Response = SaveGeometryToFileRes;

    fn request_type_name(&self) -> &str { "SaveGeometryToFileReq" }
    fn response_type_name(&self) -> &str { "SaveGeometryToFileRes" }
}
