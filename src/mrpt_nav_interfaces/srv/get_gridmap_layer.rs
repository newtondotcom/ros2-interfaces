use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridmapLayerReq {
    pub layer_name: ::std::string::String,
}

impl Default for GetGridmapLayerReq {
    fn default() -> Self {
        GetGridmapLayerReq {
            layer_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetGridmapLayerReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridmapLayerRes {
    pub valid: bool,
    pub grid: crate::nav_msgs::msg::OccupancyGrid,
}

impl Default for GetGridmapLayerRes {
    fn default() -> Self {
        GetGridmapLayerRes {
            valid: false,
            grid: crate::nav_msgs::msg::OccupancyGrid::default(),
        }
    }
}

impl ros2_client::Message for GetGridmapLayerRes {}


pub struct GetGridmapLayer;
impl ros2_client::Service for GetGridmapLayer {
    type Request = GetGridmapLayerReq;
    type Response = GetGridmapLayerRes;

    fn request_type_name(&self) -> &str { "GetGridmapLayerReq" }
    fn response_type_name(&self) -> &str { "GetGridmapLayerRes" }
}
