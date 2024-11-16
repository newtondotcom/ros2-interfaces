use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnNodeFromStringReq {
    pub data: ::std::string::String,
}

impl Default for SpawnNodeFromStringReq {
    fn default() -> Self {
        SpawnNodeFromStringReq {
            data: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpawnNodeFromStringReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnNodeFromStringRes {
    pub success: bool,
}

impl Default for SpawnNodeFromStringRes {
    fn default() -> Self {
        SpawnNodeFromStringRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SpawnNodeFromStringRes {}


pub struct SpawnNodeFromString;
impl ros2_client::Service for SpawnNodeFromString {
    type Request = SpawnNodeFromStringReq;
    type Response = SpawnNodeFromStringRes;

    fn request_type_name(&self) -> &str { "SpawnNodeFromStringReq" }
    fn response_type_name(&self) -> &str { "SpawnNodeFromStringRes" }
}
