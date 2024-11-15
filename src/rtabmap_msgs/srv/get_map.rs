use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMapReq {
    pub global_map: bool,
    pub optimized: bool,
    pub graph_only: bool,
}

impl Default for GetMapReq {
    fn default() -> Self {
        GetMapReq {
            global_map: false,
            optimized: false,
            graph_only: false,
        }
    }
}

impl ros2_client::Message for GetMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMapRes {
    pub data: crate::rtabmap_msgs::msg::MapData,
}

impl Default for GetMapRes {
    fn default() -> Self {
        GetMapRes {
            data: crate::rtabmap_msgs::msg::MapData::default(),
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
