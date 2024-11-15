use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalManagerSetRoiReq {
    pub mode: u8,
    pub gimbal_device_id: u8,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
    pub pitch_offset: f32,
    pub roll_offset: f32,
    pub yaw_offset: f32,
    pub sysid: u8,
}

impl GimbalManagerSetRoiReq {
    pub const ROI_MODE_LOCATION: u8 = 0;
    pub const ROI_MODE_WP_NEXT_OFFSET: u8 = 1;
    pub const ROI_MODE_SYSID: u8 = 2;
    pub const ROI_MODE_NONE: u8 = 3;
}

impl Default for GimbalManagerSetRoiReq {
    fn default() -> Self {
        GimbalManagerSetRoiReq {
            mode: 0,
            gimbal_device_id: 0,
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            pitch_offset: 0.0,
            roll_offset: 0.0,
            yaw_offset: 0.0,
            sysid: 0,
        }
    }
}

impl ros2_client::Message for GimbalManagerSetRoiReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalManagerSetRoiRes {
    pub success: bool,
    pub result: u8,
}

impl Default for GimbalManagerSetRoiRes {
    fn default() -> Self {
        GimbalManagerSetRoiRes {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for GimbalManagerSetRoiRes {}


pub struct GimbalManagerSetRoi;
impl ros2_client::Service for GimbalManagerSetRoi {
    type Request = GimbalManagerSetRoiReq;
    type Response = GimbalManagerSetRoiRes;

    fn request_type_name(&self) -> &str { "GimbalManagerSetRoiReq" }
    fn response_type_name(&self) -> &str { "GimbalManagerSetRoiRes" }
}
