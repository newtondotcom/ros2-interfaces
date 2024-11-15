use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlModeCommandReq {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub mode: u8,
}

impl ControlModeCommandReq {
    pub const NO_COMMAND: u8 = 0;
    pub const AUTONOMOUS: u8 = 1;
    pub const AUTONOMOUS_STEER_ONLY: u8 = 2;
    pub const AUTONOMOUS_VELOCITY_ONLY: u8 = 3;
    pub const MANUAL: u8 = 4;
}

impl Default for ControlModeCommandReq {
    fn default() -> Self {
        ControlModeCommandReq {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            mode: 0,
        }
    }
}

impl ros2_client::Message for ControlModeCommandReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlModeCommandRes {
    pub success: bool,
}

impl Default for ControlModeCommandRes {
    fn default() -> Self {
        ControlModeCommandRes {
            success: false,
        }
    }
}

impl ros2_client::Message for ControlModeCommandRes {}


pub struct ControlModeCommand;
impl ros2_client::Service for ControlModeCommand {
    type Request = ControlModeCommandReq;
    type Response = ControlModeCommandRes;

    fn request_type_name(&self) -> &str { "ControlModeCommandReq" }
    fn response_type_name(&self) -> &str { "ControlModeCommandRes" }
}
