use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGridMapReq {
    pub map: crate::grid_map_msgs::msg::GridMap,
}

impl Default for SetGridMapReq {
    fn default() -> Self {
        SetGridMapReq {
            map: crate::grid_map_msgs::msg::GridMap::default(),
        }
    }
}

impl ros2_client::Message for SetGridMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGridMapRes {

}

impl Default for SetGridMapRes {
    fn default() -> Self {
        SetGridMapRes {

        }
    }
}

impl ros2_client::Message for SetGridMapRes {}


pub struct SetGridMap;
impl ros2_client::Service for SetGridMap {
    type Request = SetGridMapReq;
    type Response = SetGridMapRes;

    fn request_type_name(&self) -> &str { "SetGridMapReq" }
    fn response_type_name(&self) -> &str { "SetGridMapRes" }
}
