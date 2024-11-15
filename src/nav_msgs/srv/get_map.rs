use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMapReq {

}

impl Default for GetMapReq {
    fn default() -> Self {
        GetMapReq {

        }
    }
}

impl ros2_client::Message for GetMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMapRes {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
}

impl Default for GetMapRes {
    fn default() -> Self {
        GetMapRes {
            map: crate::nav_msgs::msg::OccupancyGrid::default(),
        }
    }
}

impl ros2_client::Message for GetMapRes {}


pub struct GetMap;
impl ros2_client::Service for GetMap {
    type Request = GetMapReq;
    type Response = GetMapRes;

    fn request_type_name(&self) -> &str { "GetMapReq" }
    fn response_type_name(&self) -> &str { "GetMapRes" }
}
