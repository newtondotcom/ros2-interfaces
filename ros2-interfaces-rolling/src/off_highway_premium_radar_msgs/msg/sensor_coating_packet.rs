use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorCoatingPacket {
    pub theta_indicator_mimo: f32,
    pub theta_indicator_mimo_valid: bool,
    pub phi_indicator: f32,
    pub phi_indicator_valid: bool,
    pub reflections_indicator: f32,
    pub reflections_indicator_valid: bool,
    pub theta_mimo_rate: f32,
    pub theta_mimo_rate_valid: bool,
}

impl Default for SensorCoatingPacket {
    fn default() -> Self {
        SensorCoatingPacket {
            theta_indicator_mimo: 0.0,
            theta_indicator_mimo_valid: false,
            phi_indicator: 0.0,
            phi_indicator_valid: false,
            reflections_indicator: 0.0,
            reflections_indicator_valid: false,
            theta_mimo_rate: 0.0,
            theta_mimo_rate_valid: false,
        }
    }
}

impl ros2_client::Message for SensorCoatingPacket {}
