use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FromLLReq {
    pub ll_point: crate::geographic_msgs::msg::GeoPoint,
}

impl Default for FromLLReq {
    fn default() -> Self {
        FromLLReq {
            ll_point: crate::geographic_msgs::msg::GeoPoint::default(),
        }
    }
}

impl ros2_client::Message for FromLLReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FromLLRes {
    pub map_point: crate::geometry_msgs::msg::Point,
}

impl Default for FromLLRes {
    fn default() -> Self {
        FromLLRes {
            map_point: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

impl ros2_client::Message for FromLLRes {}


pub struct FromLL;
impl ros2_client::Service for FromLL {
    type Request = FromLLReq;
    type Response = FromLLRes;

    fn request_type_name(&self) -> &str { "FromLLReq" }
    fn response_type_name(&self) -> &str { "FromLLRes" }
}
