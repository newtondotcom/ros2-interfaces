use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorFieldOfView {
    #[serde_as(as = "[_; 99]")]
    pub fov: [f32; 99],
    #[serde_as(as = "[_; 99]")]
    pub azimuth: [f32; 99],
    pub elevation_range_scaling: [f32; 11],
    pub elevation: [f32; 11],
}

impl Default for SensorFieldOfView {
    fn default() -> Self {
        SensorFieldOfView {
            fov: [0.0; 99],
            azimuth: [0.0; 99],
            elevation_range_scaling: [0.0; 11],
            elevation: [0.0; 11],
        }
    }
}

impl ros2_client::Message for SensorFieldOfView {}
