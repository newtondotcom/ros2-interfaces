use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRouteListReq {

}

impl Default for GetRouteListReq {
    fn default() -> Self {
        GetRouteListReq {

        }
    }
}

impl ros2_client::Message for GetRouteListReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRouteListRes {
    pub routes: Vec<crate::marti_nav_msgs::msg::Route>,
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for GetRouteListRes {
    fn default() -> Self {
        GetRouteListRes {
            routes: Vec::new(),
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetRouteListRes {}


pub struct GetRouteList;
impl ros2_client::Service for GetRouteList {
    type Request = GetRouteListReq;
    type Response = GetRouteListRes;

    fn request_type_name(&self) -> &str { "GetRouteListReq" }
    fn response_type_name(&self) -> &str { "GetRouteListRes" }
}
