use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPointmapLayerReq {
    pub layer_name: ::std::string::String,
}

impl Default for GetPointmapLayerReq {
    fn default() -> Self {
        GetPointmapLayerReq {
            layer_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetPointmapLayerReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPointmapLayerRes {
    pub valid: bool,
    pub points: crate::sensor_msgs::msg::PointCloud2,
}

impl Default for GetPointmapLayerRes {
    fn default() -> Self {
        GetPointmapLayerRes {
            valid: false,
            points: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}

impl ros2_client::Message for GetPointmapLayerRes {}


pub struct GetPointmapLayer;
impl ros2_client::Service for GetPointmapLayer {
    type Request = GetPointmapLayerReq;
    type Response = GetPointmapLayerRes;

    fn request_type_name(&self) -> &str { "GetPointmapLayerReq" }
    fn response_type_name(&self) -> &str { "GetPointmapLayerRes" }
}
