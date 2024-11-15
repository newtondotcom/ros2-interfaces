use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapLoadReq {
    pub map_path: ::std::string::String,
}

impl Default for MapLoadReq {
    fn default() -> Self {
        MapLoadReq {
            map_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MapLoadReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapLoadRes {
    pub success: bool,
    pub error_message: ::std::string::String,
}

impl Default for MapLoadRes {
    fn default() -> Self {
        MapLoadRes {
            success: false,
            error_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MapLoadRes {}


pub struct MapLoad;
impl ros2_client::Service for MapLoad {
    type Request = MapLoadReq;
    type Response = MapLoadRes;

    fn request_type_name(&self) -> &str { "MapLoadReq" }
    fn response_type_name(&self) -> &str { "MapLoadRes" }
}
