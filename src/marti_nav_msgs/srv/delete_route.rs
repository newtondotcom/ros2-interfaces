use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRouteReq {
    pub guid: ::std::string::String,
}

impl Default for DeleteRouteReq {
    fn default() -> Self {
        DeleteRouteReq {
            guid: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteRouteReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRouteRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for DeleteRouteRes {
    fn default() -> Self {
        DeleteRouteRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteRouteRes {}


pub struct DeleteRoute;
impl ros2_client::Service for DeleteRoute {
    type Request = DeleteRouteReq;
    type Response = DeleteRouteRes;

    fn request_type_name(&self) -> &str { "DeleteRouteReq" }
    fn response_type_name(&self) -> &str { "DeleteRouteRes" }
}
