use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapSaveReq {
    pub map_path: ::std::string::String,
}

impl Default for MapSaveReq {
    fn default() -> Self {
        MapSaveReq {
            map_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MapSaveReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapSaveRes {
    pub success: bool,
    pub error_message: ::std::string::String,
}

impl Default for MapSaveRes {
    fn default() -> Self {
        MapSaveRes {
            success: false,
            error_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MapSaveRes {}


pub struct MapSave;
impl ros2_client::Service for MapSave {
    type Request = MapSaveReq;
    type Response = MapSaveRes;

    fn request_type_name(&self) -> &str { "MapSaveReq" }
    fn response_type_name(&self) -> &str { "MapSaveRes" }
}
