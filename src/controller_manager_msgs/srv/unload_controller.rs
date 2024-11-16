use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnloadControllerReq {
    pub name: ::std::string::String,
}

impl Default for UnloadControllerReq {
    fn default() -> Self {
        UnloadControllerReq {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UnloadControllerReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnloadControllerRes {
    pub ok: bool,
}

impl Default for UnloadControllerRes {
    fn default() -> Self {
        UnloadControllerRes {
            ok: false,
        }
    }
}

impl ros2_client::Message for UnloadControllerRes {}


pub struct UnloadController;
impl ros2_client::Service for UnloadController {
    type Request = UnloadControllerReq;
    type Response = UnloadControllerRes;

    fn request_type_name(&self) -> &str { "UnloadControllerReq" }
    fn response_type_name(&self) -> &str { "UnloadControllerRes" }
}
