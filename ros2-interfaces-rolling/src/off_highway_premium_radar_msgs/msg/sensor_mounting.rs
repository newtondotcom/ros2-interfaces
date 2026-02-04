use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorMounting {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub azimuth: f32,
    pub elevation: f32,
    pub orientation: i8,
}

impl Default for SensorMounting {
    fn default() -> Self {
        SensorMounting {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            azimuth: 0.0,
            elevation: 0.0,
            orientation: 0,
        }
    }
}

impl ros2_client::Message for SensorMounting {}
