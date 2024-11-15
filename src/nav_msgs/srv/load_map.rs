use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadMapReq {
    pub map_url: ::std::string::String,
}

impl Default for LoadMapReq {
    fn default() -> Self {
        LoadMapReq {
            map_url: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LoadMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadMapRes {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
    pub result: u8,
}

impl LoadMapRes {
    pub const RESULT_SUCCESS: u8 = 0;
    pub const RESULT_MAP_DOES_NOT_EXIST: u8 = 1;
    pub const RESULT_INVALID_MAP_DATA: u8 = 2;
    pub const RESULT_INVALID_MAP_METADATA: u8 = 3;
    pub const RESULT_UNDEFINED_FAILURE: u8 = 255;
}

impl Default for LoadMapRes {
    fn default() -> Self {
        LoadMapRes {
            map: crate::nav_msgs::msg::OccupancyGrid::default(),
            result: 0,
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
