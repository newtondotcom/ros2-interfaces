use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectMoreLoopClosuresReq {
    pub cluster_radius_max: f32,
    pub cluster_radius_min: f32,
    pub cluster_angle: f32,
    pub iterations: i32,
    pub intra_only: bool,
    pub inter_only: bool,
}

impl Default for DetectMoreLoopClosuresReq {
    fn default() -> Self {
        DetectMoreLoopClosuresReq {
            cluster_radius_max: 0.0,
            cluster_radius_min: 0.0,
            cluster_angle: 0.0,
            iterations: 0,
            intra_only: false,
            inter_only: false,
        }
    }
}

impl ros2_client::Message for DetectMoreLoopClosuresReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectMoreLoopClosuresRes {
    pub detected: i32,
}

impl Default for DetectMoreLoopClosuresRes {
    fn default() -> Self {
        DetectMoreLoopClosuresRes {
            detected: 0,
        }
    }
}

impl ros2_client::Message for DetectMoreLoopClosuresRes {}


pub struct DetectMoreLoopClosures;
impl ros2_client::Service for DetectMoreLoopClosures {
    type Request = DetectMoreLoopClosuresReq;
    type Response = DetectMoreLoopClosuresRes;

    fn request_type_name(&self) -> &str { "DetectMoreLoopClosuresReq" }
    fn response_type_name(&self) -> &str { "DetectMoreLoopClosuresRes" }
}
