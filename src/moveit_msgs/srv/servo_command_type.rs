use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServoCommandTypeReq {
    pub command_type: i8,
}

impl ServoCommandTypeReq {
    pub const JOINT_JOG: i8 = 0;
    pub const TWIST: i8 = 1;
    pub const POSE: i8 = 2;
}

impl Default for ServoCommandTypeReq {
    fn default() -> Self {
        ServoCommandTypeReq {
            command_type: 0,
        }
    }
}

impl ros2_client::Message for ServoCommandTypeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServoCommandTypeRes {
    pub success: bool,
}

impl Default for ServoCommandTypeRes {
    fn default() -> Self {
        ServoCommandTypeRes {
            success: false,
        }
    }
}

impl ros2_client::Message for ServoCommandTypeRes {}


pub struct ServoCommandType;
impl ros2_client::Service for ServoCommandType {
    type Request = ServoCommandTypeReq;
    type Response = ServoCommandTypeRes;

    fn request_type_name(&self) -> &str { "ServoCommandTypeReq" }
    fn response_type_name(&self) -> &str { "ServoCommandTypeRes" }
}
