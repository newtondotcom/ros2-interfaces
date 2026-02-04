use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bbox2D {
    pub top_left_x: f64,
    pub top_left_y: f64,
    pub bottom_right_x: f64,
    pub bottom_right_y: f64,
}

impl Default for Bbox2D {
    fn default() -> Self {
        Bbox2D {
            top_left_x: 0.0,
            top_left_y: 0.0,
            bottom_right_x: 0.0,
            bottom_right_y: 0.0,
        }
    }
}

impl ros2_client::Message for Bbox2D {}
