use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRouteReq {
    pub guid: ::std::string::String,
    pub repeat: bool,
}

impl Default for SetRouteReq {
    fn default() -> Self {
        SetRouteReq {
            guid: ::std::string::String::new(),
            repeat: false,
        }
    }
}

impl ros2_client::Message for SetRouteReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRouteRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetRouteRes {
    fn default() -> Self {
        SetRouteRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetRouteRes {}


pub struct SetRoute;
impl ros2_client::Service for SetRoute {
    type Request = SetRouteReq;
    type Response = SetRouteRes;

    fn request_type_name(&self) -> &str { "SetRouteReq" }
    fn response_type_name(&self) -> &str { "SetRouteRes" }
}
