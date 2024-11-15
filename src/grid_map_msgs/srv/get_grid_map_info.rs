use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridMapInfoReq {

}

impl Default for GetGridMapInfoReq {
    fn default() -> Self {
        GetGridMapInfoReq {

        }
    }
}

impl ros2_client::Message for GetGridMapInfoReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridMapInfoRes {
    pub info: crate::grid_map_msgs::msg::GridMapInfo,
}

impl Default for GetGridMapInfoRes {
    fn default() -> Self {
        GetGridMapInfoRes {
            info: crate::grid_map_msgs::msg::GridMapInfo::default(),
        }
    }
}

impl ros2_client::Message for GetGridMapInfoRes {}


pub struct GetGridMapInfo;
impl ros2_client::Service for GetGridMapInfo {
    type Request = GetGridMapInfoReq;
    type Response = GetGridMapInfoRes;

    fn request_type_name(&self) -> &str { "GetGridMapInfoReq" }
    fn response_type_name(&self) -> &str { "GetGridMapInfoRes" }
}
