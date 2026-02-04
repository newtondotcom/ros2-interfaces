use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VelocityLimitConstraints {
    pub max_acceleration: f32,
    pub min_acceleration: f32,
    pub max_jerk: f32,
    pub min_jerk: f32,
}

impl Default for VelocityLimitConstraints {
    fn default() -> Self {
        VelocityLimitConstraints {
            max_acceleration: 0.0,
            min_acceleration: 0.0,
            max_jerk: 0.0,
            min_jerk: 0.0,
        }
    }
}

impl ros2_client::Message for VelocityLimitConstraints {}
