use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObjectsRequest {

}

impl Default for GetObjectsRequest {
    fn default() -> Self {
        GetObjectsRequest {

        }
    }
}

impl ros2_client::Message for GetObjectsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObjectsResponse {
    pub map: crate::tuw_object_map_msgs::msg::Objects,
}

impl Default for GetObjectsResponse {
    fn default() -> Self {
        GetObjectsResponse {
            map: crate::tuw_object_map_msgs::msg::Objects::default(),
        }
    }
}

impl ros2_client::Message for GetObjectsResponse {}


pub struct GetObjects;
impl ros2_client::Service for GetObjects {
    type Request = GetObjectsRequest;
    type Response = GetObjectsResponse;

    fn request_type_name(&self) -> &str { "GetObjectsRequest" }
    fn response_type_name(&self) -> &str { "GetObjectsResponse" }
}
