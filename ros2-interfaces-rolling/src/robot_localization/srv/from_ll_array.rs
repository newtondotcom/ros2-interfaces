use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FromLLArrayRequest {
    pub ll_points: Vec<crate::geographic_msgs::msg::GeoPoint>,
}

impl Default for FromLLArrayRequest {
    fn default() -> Self {
        FromLLArrayRequest {
            ll_points: Vec::new(),
        }
    }
}

impl ros2_client::Message for FromLLArrayRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FromLLArrayResponse {
    pub map_points: Vec<crate::geometry_msgs::msg::Point>,
}

impl Default for FromLLArrayResponse {
    fn default() -> Self {
        FromLLArrayResponse {
            map_points: Vec::new(),
        }
    }
}

impl ros2_client::Message for FromLLArrayResponse {}


pub struct FromLLArray;
impl ros2_client::Service for FromLLArray {
    type Request = FromLLArrayRequest;
    type Response = FromLLArrayResponse;

    fn request_type_name(&self) -> &str { "FromLLArrayRequest" }
    fn response_type_name(&self) -> &str { "FromLLArrayResponse" }
}
