use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadGeometryFromFileReq {
    pub file_path_and_name: ::std::string::String,
}

impl Default for LoadGeometryFromFileReq {
    fn default() -> Self {
        LoadGeometryFromFileReq {
            file_path_and_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LoadGeometryFromFileReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadGeometryFromFileRes {
    pub success: bool,
}

impl Default for LoadGeometryFromFileRes {
    fn default() -> Self {
        LoadGeometryFromFileRes {
            success: false,
        }
    }
}

impl ros2_client::Message for LoadGeometryFromFileRes {}


pub struct LoadGeometryFromFile;
impl ros2_client::Service for LoadGeometryFromFile {
    type Request = LoadGeometryFromFileReq;
    type Response = LoadGeometryFromFileRes;

    fn request_type_name(&self) -> &str { "LoadGeometryFromFileReq" }
    fn response_type_name(&self) -> &str { "LoadGeometryFromFileRes" }
}
