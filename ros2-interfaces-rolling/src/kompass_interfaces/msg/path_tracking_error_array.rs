use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathTrackingErrorArray {
    pub orientation_errors: Vec<f64>,
    pub lateral_distance_errors: Vec<f64>,
}

impl Default for PathTrackingErrorArray {
    fn default() -> Self {
        PathTrackingErrorArray {
            orientation_errors: Vec::new(),
            lateral_distance_errors: Vec::new(),
        }
    }
}

impl ros2_client::Message for PathTrackingErrorArray {}
