use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetModeReq {
    pub base_mode: u8,
    pub custom_mode: ::std::string::String,
}

impl SetModeReq {
    pub const MAV_MODE_PREFLIGHT: u8 = 0;
    pub const MAV_MODE_STABILIZE_DISARMED: u8 = 80;
    pub const MAV_MODE_STABILIZE_ARMED: u8 = 208;
    pub const MAV_MODE_MANUAL_DISARMED: u8 = 64;
    pub const MAV_MODE_MANUAL_ARMED: u8 = 192;
    pub const MAV_MODE_GUIDED_DISARMED: u8 = 88;
    pub const MAV_MODE_GUIDED_ARMED: u8 = 216;
    pub const MAV_MODE_AUTO_DISARMED: u8 = 92;
    pub const MAV_MODE_AUTO_ARMED: u8 = 220;
    pub const MAV_MODE_TEST_DISARMED: u8 = 66;
    pub const MAV_MODE_TEST_ARMED: u8 = 194;
}

impl Default for SetModeReq {
    fn default() -> Self {
        SetModeReq {
            base_mode: 0,
            custom_mode: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetModeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetModeRes {
    pub mode_sent: bool,
}

impl Default for SetModeRes {
    fn default() -> Self {
        SetModeRes {
            mode_sent: false,
        }
    }
}

impl ros2_client::Message for SetModeRes {}


pub struct SetMode;
impl ros2_client::Service for SetMode {
    type Request = SetModeReq;
    type Response = SetModeRes;

    fn request_type_name(&self) -> &str { "SetModeReq" }
    fn response_type_name(&self) -> &str { "SetModeRes" }
}
