use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveRouteReq {
    pub name: ::std::string::String,
    pub guid: ::std::string::String,
    pub route: crate::marti_nav_msgs::msg::Route,
    pub thumbnail: ::std::string::String,
}

impl Default for SaveRouteReq {
    fn default() -> Self {
        SaveRouteReq {
            name: ::std::string::String::new(),
            guid: ::std::string::String::new(),
            route: crate::marti_nav_msgs::msg::Route::default(),
            thumbnail: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveRouteReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveRouteRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SaveRouteRes {
    fn default() -> Self {
        SaveRouteRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveRouteRes {}


pub struct SaveRoute;
impl ros2_client::Service for SaveRoute {
    type Request = SaveRouteReq;
    type Response = SaveRouteRes;

    fn request_type_name(&self) -> &str { "SaveRouteReq" }
    fn response_type_name(&self) -> &str { "SaveRouteRes" }
}
