use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetImageForMapTileReq {
    pub tile_id: ::std::string::String,
}

impl Default for GetImageForMapTileReq {
    fn default() -> Self {
        GetImageForMapTileReq {
            tile_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetImageForMapTileReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetImageForMapTileRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for GetImageForMapTileRes {
    fn default() -> Self {
        GetImageForMapTileRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetImageForMapTileRes {}


pub struct GetImageForMapTile;
impl ros2_client::Service for GetImageForMapTile {
    type Request = GetImageForMapTileReq;
    type Response = GetImageForMapTileRes;

    fn request_type_name(&self) -> &str { "GetImageForMapTileReq" }
    fn response_type_name(&self) -> &str { "GetImageForMapTileRes" }
}
