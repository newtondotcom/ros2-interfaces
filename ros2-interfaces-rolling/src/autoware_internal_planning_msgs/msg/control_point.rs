use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlPoint {
    pub pose: crate::geometry_msgs::msg::Pose,
    pub velocity: f32,
    pub shift_length: f32,
    pub distance: f32,
}

impl Default for ControlPoint {
    fn default() -> Self {
        ControlPoint {
            pose: crate::geometry_msgs::msg::Pose::default(),
            velocity: 0.0,
            shift_length: 0.0,
            distance: 0.0,
        }
    }
}

impl ros2_client::Message for ControlPoint {}
