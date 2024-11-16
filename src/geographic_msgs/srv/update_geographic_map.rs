use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateGeographicMapReq {
    pub updates: crate::geographic_msgs::msg::GeographicMapChanges,
}

impl Default for UpdateGeographicMapReq {
    fn default() -> Self {
        UpdateGeographicMapReq {
            updates: crate::geographic_msgs::msg::GeographicMapChanges::default(),
        }
    }
}

impl ros2_client::Message for UpdateGeographicMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateGeographicMapRes {
    pub success: bool,
    pub status: ::std::string::String,
}

impl Default for UpdateGeographicMapRes {
    fn default() -> Self {
        UpdateGeographicMapRes {
            success: false,
            status: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UpdateGeographicMapRes {}


pub struct UpdateGeographicMap;
impl ros2_client::Service for UpdateGeographicMap {
    type Request = UpdateGeographicMapReq;
    type Response = UpdateGeographicMapRes;

    fn request_type_name(&self) -> &str { "UpdateGeographicMapReq" }
    fn response_type_name(&self) -> &str { "UpdateGeographicMapRes" }
}
