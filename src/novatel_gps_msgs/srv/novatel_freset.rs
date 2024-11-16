use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelFRESETReq {
    pub target: ::std::string::String,
}

impl Default for NovatelFRESETReq {
    fn default() -> Self {
        NovatelFRESETReq {
            target: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for NovatelFRESETReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelFRESETRes {
    pub success: bool,
}

impl Default for NovatelFRESETRes {
    fn default() -> Self {
        NovatelFRESETRes {
            success: false,
        }
    }
}

impl ros2_client::Message for NovatelFRESETRes {}


pub struct NovatelFRESET;
impl ros2_client::Service for NovatelFRESET {
    type Request = NovatelFRESETReq;
    type Response = NovatelFRESETRes;

    fn request_type_name(&self) -> &str { "NovatelFRESETReq" }
    fn response_type_name(&self) -> &str { "NovatelFRESETRes" }
}
