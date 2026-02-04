use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TwistArray {
    pub linear_velocities: crate::kompass_interfaces::msg::VectorArray,
    pub angular_velocities: crate::kompass_interfaces::msg::VectorArray,
    pub time_step: f32,
}

impl Default for TwistArray {
    fn default() -> Self {
        TwistArray {
            linear_velocities: crate::kompass_interfaces::msg::VectorArray::default(),
            angular_velocities: crate::kompass_interfaces::msg::VectorArray::default(),
            time_step: 0.0,
        }
    }
}

impl ros2_client::Message for TwistArray {}
