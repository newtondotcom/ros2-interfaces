use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveGridReq {
    pub file_name: ::std::string::String,
}

impl Default for SaveGridReq {
    fn default() -> Self {
        SaveGridReq {
            file_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveGridReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveGridRes {
    pub map_size_bytes: f64,
    pub status: bool,
}

impl Default for SaveGridRes {
    fn default() -> Self {
        SaveGridRes {
            map_size_bytes: 0.0,
            status: false,
        }
    }
}

impl ros2_client::Message for SaveGridRes {}


pub struct SaveGrid;
impl ros2_client::Service for SaveGrid {
    type Request = SaveGridReq;
    type Response = SaveGridRes;

    fn request_type_name(&self) -> &str { "SaveGridReq" }
    fn response_type_name(&self) -> &str { "SaveGridRes" }
}
