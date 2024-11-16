use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlWorldReq {
    pub world_control: crate::ros_gz_interfaces::msg::WorldControl,
}

impl Default for ControlWorldReq {
    fn default() -> Self {
        ControlWorldReq {
            world_control: crate::ros_gz_interfaces::msg::WorldControl::default(),
        }
    }
}

impl ros2_client::Message for ControlWorldReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlWorldRes {
    pub success: bool,
}

impl Default for ControlWorldRes {
    fn default() -> Self {
        ControlWorldRes {
            success: false,
        }
    }
}

impl ros2_client::Message for ControlWorldRes {}


pub struct ControlWorld;
impl ros2_client::Service for ControlWorld {
    type Request = ControlWorldReq;
    type Response = ControlWorldRes;

    fn request_type_name(&self) -> &str { "ControlWorldReq" }
    fn response_type_name(&self) -> &str { "ControlWorldRes" }
}
