use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalManagerConfigureReq {
    pub sysid_primary: i16,
    pub compid_primary: i16,
    pub sysid_secondary: i16,
    pub compid_secondary: i16,
    pub gimbal_device_id: u8,
}

impl Default for GimbalManagerConfigureReq {
    fn default() -> Self {
        GimbalManagerConfigureReq {
            sysid_primary: 0,
            compid_primary: 0,
            sysid_secondary: 0,
            compid_secondary: 0,
            gimbal_device_id: 0,
        }
    }
}

impl ros2_client::Message for GimbalManagerConfigureReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalManagerConfigureRes {
    pub success: bool,
    pub result: u8,
}

impl Default for GimbalManagerConfigureRes {
    fn default() -> Self {
        GimbalManagerConfigureRes {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for GimbalManagerConfigureRes {}


pub struct GimbalManagerConfigure;
impl ros2_client::Service for GimbalManagerConfigure {
    type Request = GimbalManagerConfigureReq;
    type Response = GimbalManagerConfigureRes;

    fn request_type_name(&self) -> &str { "GimbalManagerConfigureReq" }
    fn response_type_name(&self) -> &str { "GimbalManagerConfigureRes" }
}
