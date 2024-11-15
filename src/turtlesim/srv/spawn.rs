use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnReq {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub name: ::std::string::String,
}

impl Default for SpawnReq {
    fn default() -> Self {
        SpawnReq {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpawnReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnRes {
    pub name: ::std::string::String,
}

impl Default for SpawnRes {
    fn default() -> Self {
        SpawnRes {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpawnRes {}


pub struct Spawn;
impl ros2_client::Service for Spawn {
    type Request = SpawnReq;
    type Response = SpawnRes;

    fn request_type_name(&self) -> &str { "SpawnReq" }
    fn response_type_name(&self) -> &str { "SpawnRes" }
}
