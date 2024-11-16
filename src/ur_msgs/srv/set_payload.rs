use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPayloadReq {
    pub mass: f32,
    pub center_of_gravity: crate::geometry_msgs::msg::Vector3,
}

impl Default for SetPayloadReq {
    fn default() -> Self {
        SetPayloadReq {
            mass: 0.0,
            center_of_gravity: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for SetPayloadReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPayloadRes {
    pub success: bool,
}

impl Default for SetPayloadRes {
    fn default() -> Self {
        SetPayloadRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SetPayloadRes {}


pub struct SetPayload;
impl ros2_client::Service for SetPayload {
    type Request = SetPayloadReq;
    type Response = SetPayloadRes;

    fn request_type_name(&self) -> &str { "SetPayloadReq" }
    fn response_type_name(&self) -> &str { "SetPayloadRes" }
}
