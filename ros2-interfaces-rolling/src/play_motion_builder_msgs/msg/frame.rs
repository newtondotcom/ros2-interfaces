use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub pose: Vec<f64>,
    pub time_from_last: f32,
}

impl Default for Frame {
    fn default() -> Self {
        Frame {
            pose: Vec::new(),
            time_from_last: 0.0,
        }
    }
}

impl ros2_client::Message for Frame {}
