use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPointMapReq {

}

impl Default for GetPointMapReq {
    fn default() -> Self {
        GetPointMapReq {

        }
    }
}

impl ros2_client::Message for GetPointMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPointMapRes {
    pub map: crate::sensor_msgs::msg::PointCloud2,
}

impl Default for GetPointMapRes {
    fn default() -> Self {
        GetPointMapRes {
            map: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}

impl ros2_client::Message for GetPointMapRes {}


pub struct GetPointMap;
impl ros2_client::Service for GetPointMap {
    type Request = GetPointMapReq;
    type Response = GetPointMapRes;

    fn request_type_name(&self) -> &str { "GetPointMapReq" }
    fn response_type_name(&self) -> &str { "GetPointMapRes" }
}
