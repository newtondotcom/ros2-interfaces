use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictedOccupancies {
    pub header: crate::std_msgs::msg::Header,
    pub start_time: f32,
    pub horizon: f32,
    pub time_step: f32,
    pub obstacles: Vec<crate::kompass_interfaces::msg::Obstacles>,
}

impl Default for PredictedOccupancies {
    fn default() -> Self {
        PredictedOccupancies {
            header: crate::std_msgs::msg::Header::default(),
            start_time: 0.0,
            horizon: 0.0,
            time_step: 0.0,
            obstacles: Vec::new(),
        }
    }
}

impl ros2_client::Message for PredictedOccupancies {}
