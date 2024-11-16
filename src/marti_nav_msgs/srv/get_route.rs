use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRouteReq {
    pub guid: ::std::string::String,
}

impl Default for GetRouteReq {
    fn default() -> Self {
        GetRouteReq {
            guid: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetRouteReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRouteRes {
    pub route: crate::marti_nav_msgs::msg::Route,
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for GetRouteRes {
    fn default() -> Self {
        GetRouteRes {
            route: crate::marti_nav_msgs::msg::Route::default(),
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetRouteRes {}


pub struct GetRoute;
impl ros2_client::Service for GetRoute {
    type Request = GetRouteReq;
    type Response = GetRouteRes;

    fn request_type_name(&self) -> &str { "GetRouteReq" }
    fn response_type_name(&self) -> &str { "GetRouteRes" }
}
