use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMapReq {
    pub filename: ::std::string::String,
}

impl Default for SaveMapReq {
    fn default() -> Self {
        SaveMapReq {
            filename: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMapRes {
    pub success: bool,
}

impl Default for SaveMapRes {
    fn default() -> Self {
        SaveMapRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SaveMapRes {}


pub struct SaveMap;
impl ros2_client::Service for SaveMap {
    type Request = SaveMapReq;
    type Response = SaveMapRes;

    fn request_type_name(&self) -> &str { "SaveMapReq" }
    fn response_type_name(&self) -> &str { "SaveMapRes" }
}
