use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadControllerReq {
    pub name: ::std::string::String,
}

impl Default for LoadControllerReq {
    fn default() -> Self {
        LoadControllerReq {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LoadControllerReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadControllerRes {
    pub ok: bool,
}

impl Default for LoadControllerRes {
    fn default() -> Self {
        LoadControllerRes {
            ok: false,
        }
    }
}

impl ros2_client::Message for LoadControllerRes {}


pub struct LoadController;
impl ros2_client::Service for LoadController {
    type Request = LoadControllerReq;
    type Response = LoadControllerRes;

    fn request_type_name(&self) -> &str { "LoadControllerReq" }
    fn response_type_name(&self) -> &str { "LoadControllerRes" }
}
