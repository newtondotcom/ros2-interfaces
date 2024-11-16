use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigureControllerReq {
    pub name: ::std::string::String,
}

impl Default for ConfigureControllerReq {
    fn default() -> Self {
        ConfigureControllerReq {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ConfigureControllerReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigureControllerRes {
    pub ok: bool,
}

impl Default for ConfigureControllerRes {
    fn default() -> Self {
        ConfigureControllerRes {
            ok: false,
        }
    }
}

impl ros2_client::Message for ConfigureControllerRes {}


pub struct ConfigureController;
impl ros2_client::Service for ConfigureController {
    type Request = ConfigureControllerReq;
    type Response = ConfigureControllerRes;

    fn request_type_name(&self) -> &str { "ConfigureControllerReq" }
    fn response_type_name(&self) -> &str { "ConfigureControllerRes" }
}
