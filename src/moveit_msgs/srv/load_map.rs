use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadMapReq {
    pub filename: ::std::string::String,
}

impl Default for LoadMapReq {
    fn default() -> Self {
        LoadMapReq {
            filename: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LoadMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadMapRes {
    pub success: bool,
}

impl Default for LoadMapRes {
    fn default() -> Self {
        LoadMapRes {
            success: false,
        }
    }
}

impl ros2_client::Message for LoadMapRes {}


pub struct LoadMap;
impl ros2_client::Service for LoadMap {
    type Request = LoadMapReq;
    type Response = LoadMapRes;

    fn request_type_name(&self) -> &str { "LoadMapReq" }
    fn response_type_name(&self) -> &str { "LoadMapRes" }
}
