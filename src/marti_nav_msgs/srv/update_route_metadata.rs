use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateRouteMetadataReq {
    pub route_guid: ::std::string::String,
    pub metadata_points: Vec<crate::marti_nav_msgs::msg::RoutePoint>,
}

impl Default for UpdateRouteMetadataReq {
    fn default() -> Self {
        UpdateRouteMetadataReq {
            route_guid: ::std::string::String::new(),
            metadata_points: Vec::new(),
        }
    }
}

impl ros2_client::Message for UpdateRouteMetadataReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateRouteMetadataRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for UpdateRouteMetadataRes {
    fn default() -> Self {
        UpdateRouteMetadataRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UpdateRouteMetadataRes {}


pub struct UpdateRouteMetadata;
impl ros2_client::Service for UpdateRouteMetadata {
    type Request = UpdateRouteMetadataReq;
    type Response = UpdateRouteMetadataRes;

    fn request_type_name(&self) -> &str { "UpdateRouteMetadataReq" }
    fn response_type_name(&self) -> &str { "UpdateRouteMetadataRes" }
}
