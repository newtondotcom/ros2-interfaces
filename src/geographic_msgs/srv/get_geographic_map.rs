use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGeographicMapReq {
    pub url: ::std::string::String,
    pub bounds: crate::geographic_msgs::msg::BoundingBox,
}

impl Default for GetGeographicMapReq {
    fn default() -> Self {
        GetGeographicMapReq {
            url: ::std::string::String::new(),
            bounds: crate::geographic_msgs::msg::BoundingBox::default(),
        }
    }
}

impl ros2_client::Message for GetGeographicMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGeographicMapRes {
    pub success: bool,
    pub status: ::std::string::String,
    pub map: crate::geographic_msgs::msg::GeographicMap,
}

impl Default for GetGeographicMapRes {
    fn default() -> Self {
        GetGeographicMapRes {
            success: false,
            status: ::std::string::String::new(),
            map: crate::geographic_msgs::msg::GeographicMap::default(),
        }
    }
}

impl ros2_client::Message for GetGeographicMapRes {}


pub struct GetGeographicMap;
impl ros2_client::Service for GetGeographicMap {
    type Request = GetGeographicMapReq;
    type Response = GetGeographicMapRes;

    fn request_type_name(&self) -> &str { "GetGeographicMapReq" }
    fn response_type_name(&self) -> &str { "GetGeographicMapRes" }
}
