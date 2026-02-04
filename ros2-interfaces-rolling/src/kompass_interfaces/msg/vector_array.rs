use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorArray {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
}

impl Default for VectorArray {
    fn default() -> Self {
        VectorArray {
            x: Vec::new(),
            y: Vec::new(),
            z: Vec::new(),
        }
    }
}

impl ros2_client::Message for VectorArray {}
