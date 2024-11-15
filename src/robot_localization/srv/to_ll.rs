use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToLLReq {
    pub map_point: crate::geometry_msgs::msg::Point,
}

impl Default for ToLLReq {
    fn default() -> Self {
        ToLLReq {
            map_point: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

impl ros2_client::Message for ToLLReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToLLRes {
    pub ll_point: crate::geographic_msgs::msg::GeoPoint,
}

impl Default for ToLLRes {
    fn default() -> Self {
        ToLLRes {
            ll_point: crate::geographic_msgs::msg::GeoPoint::default(),
        }
    }
}

impl ros2_client::Message for ToLLRes {}


pub struct ToLL;
impl ros2_client::Service for ToLL {
    type Request = ToLLReq;
    type Response = ToLLRes;

    fn request_type_name(&self) -> &str { "ToLLReq" }
    fn response_type_name(&self) -> &str { "ToLLRes" }
}
