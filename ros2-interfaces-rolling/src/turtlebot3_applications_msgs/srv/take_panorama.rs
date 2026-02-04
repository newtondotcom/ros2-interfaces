use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakePanoramaRequest {
    pub mode: u8,
    pub pano_angle: f32,
    pub snap_interval: f32,
    pub rot_vel: f32,
}

impl TakePanoramaRequest {
    pub const SNAPANDROTATE: u8 = 0;
    pub const CONTINUOUS: u8 = 1;
    pub const STOP: u8 = 2;
    pub const STARTED: u8 = 0;
    pub const IN_PROGRESS: u8 = 1;
    pub const STOPPED: u8 = 2;
}

impl Default for TakePanoramaRequest {
    fn default() -> Self {
        TakePanoramaRequest {
            mode: 0,
            pano_angle: 0.0,
            snap_interval: 0.0,
            rot_vel: 0.0,
        }
    }
}

impl ros2_client::Message for TakePanoramaRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakePanoramaResponse {
    pub status: u8,
}

impl Default for TakePanoramaResponse {
    fn default() -> Self {
        TakePanoramaResponse {
            status: 0,
        }
    }
}

impl ros2_client::Message for TakePanoramaResponse {}


pub struct TakePanorama;
impl ros2_client::Service for TakePanorama {
    type Request = TakePanoramaRequest;
    type Response = TakePanoramaResponse;

    fn request_type_name(&self) -> &str { "TakePanoramaRequest" }
    fn response_type_name(&self) -> &str { "TakePanoramaResponse" }
}
