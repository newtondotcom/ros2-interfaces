use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveRecordedRouteReq {
    pub name: ::std::string::String,
    pub thumbnail: ::std::string::String,
}

impl Default for SaveRecordedRouteReq {
    fn default() -> Self {
        SaveRecordedRouteReq {
            name: ::std::string::String::new(),
            thumbnail: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveRecordedRouteReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveRecordedRouteRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SaveRecordedRouteRes {
    fn default() -> Self {
        SaveRecordedRouteRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveRecordedRouteRes {}


pub struct SaveRecordedRoute;
impl ros2_client::Service for SaveRecordedRoute {
    type Request = SaveRecordedRouteReq;
    type Response = SaveRecordedRouteRes;

    fn request_type_name(&self) -> &str { "SaveRecordedRouteReq" }
    fn response_type_name(&self) -> &str { "SaveRecordedRouteRes" }
}
