use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetPoseReq {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for ResetPoseReq {
    fn default() -> Self {
        ResetPoseReq {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl ros2_client::Message for ResetPoseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetPoseRes {

}

impl Default for ResetPoseRes {
    fn default() -> Self {
        ResetPoseRes {

        }
    }
}

impl ros2_client::Message for ResetPoseRes {}


pub struct ResetPose;
impl ros2_client::Service for ResetPose {
    type Request = ResetPoseReq;
    type Response = ResetPoseRes;

    fn request_type_name(&self) -> &str { "ResetPoseReq" }
    fn response_type_name(&self) -> &str { "ResetPoseRes" }
}
