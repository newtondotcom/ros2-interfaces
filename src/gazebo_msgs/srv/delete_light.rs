use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteLightReq {
    pub light_name: ::std::string::String,
}

impl Default for DeleteLightReq {
    fn default() -> Self {
        DeleteLightReq {
            light_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteLightReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteLightRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for DeleteLightRes {
    fn default() -> Self {
        DeleteLightRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteLightRes {}


pub struct DeleteLight;
impl ros2_client::Service for DeleteLight {
    type Request = DeleteLightReq;
    type Response = DeleteLightRes;

    fn request_type_name(&self) -> &str { "DeleteLightReq" }
    fn response_type_name(&self) -> &str { "DeleteLightRes" }
}
