use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathTrackingError {
    pub orientation_error: f64,
    pub lateral_distance_error: f64,
}

impl Default for PathTrackingError {
    fn default() -> Self {
        PathTrackingError {
            orientation_error: 0.0,
            lateral_distance_error: 0.0,
        }
    }
}

impl ros2_client::Message for PathTrackingError {}
