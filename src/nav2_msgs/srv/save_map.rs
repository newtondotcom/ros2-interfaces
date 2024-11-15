use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMapReq {
    pub map_topic: ::std::string::String,
    pub map_url: ::std::string::String,
    pub image_format: ::std::string::String,
    pub map_mode: ::std::string::String,
    pub free_thresh: f32,
    pub occupied_thresh: f32,
}

impl Default for SaveMapReq {
    fn default() -> Self {
        SaveMapReq {
            map_topic: ::std::string::String::new(),
            map_url: ::std::string::String::new(),
            image_format: ::std::string::String::new(),
            map_mode: ::std::string::String::new(),
            free_thresh: 0.0,
            occupied_thresh: 0.0,
        }
    }
}

impl ros2_client::Message for SaveMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMapRes {
    pub result: bool,
}

impl Default for SaveMapRes {
    fn default() -> Self {
        SaveMapRes {
            result: false,
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
