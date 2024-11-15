use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInteractiveMarkersReq {

}

impl Default for GetInteractiveMarkersReq {
    fn default() -> Self {
        GetInteractiveMarkersReq {

        }
    }
}

impl ros2_client::Message for GetInteractiveMarkersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInteractiveMarkersRes {
    pub sequence_number: u64,
    pub markers: Vec<crate::visualization_msgs::msg::InteractiveMarker>,
}

impl Default for GetInteractiveMarkersRes {
    fn default() -> Self {
        GetInteractiveMarkersRes {
            sequence_number: 0,
            markers: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetInteractiveMarkersRes {}


pub struct GetInteractiveMarkers;
impl ros2_client::Service for GetInteractiveMarkers {
    type Request = GetInteractiveMarkersReq;
    type Response = GetInteractiveMarkersRes;

    fn request_type_name(&self) -> &str { "GetInteractiveMarkersReq" }
    fn response_type_name(&self) -> &str { "GetInteractiveMarkersRes" }
}
