use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPointMapROIReq {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub r: f64,
    pub l_x: f64,
    pub l_y: f64,
    pub l_z: f64,
}

impl Default for GetPointMapROIReq {
    fn default() -> Self {
        GetPointMapROIReq {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            r: 0.0,
            l_x: 0.0,
            l_y: 0.0,
            l_z: 0.0,
        }
    }
}

impl ros2_client::Message for GetPointMapROIReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPointMapROIRes {
    pub sub_map: crate::sensor_msgs::msg::PointCloud2,
}

impl Default for GetPointMapROIRes {
    fn default() -> Self {
        GetPointMapROIRes {
            sub_map: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}

impl ros2_client::Message for GetPointMapROIRes {}


pub struct GetPointMapROI;
impl ros2_client::Service for GetPointMapROI {
    type Request = GetPointMapROIReq;
    type Response = GetPointMapROIRes;

    fn request_type_name(&self) -> &str { "GetPointMapROIReq" }
    fn response_type_name(&self) -> &str { "GetPointMapROIRes" }
}
