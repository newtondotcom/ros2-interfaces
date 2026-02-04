use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearRouteRequest {

}

impl Default for ClearRouteRequest {
    fn default() -> Self {
        ClearRouteRequest {

        }
    }
}

impl ros2_client::Message for ClearRouteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearRouteResponse {
    pub status: crate::autoware_common_msgs::msg::ResponseStatus,
}

impl Default for ClearRouteResponse {
    fn default() -> Self {
        ClearRouteResponse {
            status: crate::autoware_common_msgs::msg::ResponseStatus::default(),
        }
    }
}

impl ros2_client::Message for ClearRouteResponse {}


pub struct ClearRoute;
impl ros2_client::Service for ClearRoute {
    type Request = ClearRouteRequest;
    type Response = ClearRouteResponse;

    fn request_type_name(&self) -> &str { "ClearRouteRequest" }
    fn response_type_name(&self) -> &str { "ClearRouteResponse" }
}
