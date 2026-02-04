use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepperState {
    pub header: crate::std_msgs::msg::Header,
    pub is_moving: bool,
    pub is_engaged: bool,
    pub target_position: f64,
}

impl Default for StepperState {
    fn default() -> Self {
        StepperState {
            header: crate::std_msgs::msg::Header::default(),
            is_moving: false,
            is_engaged: false,
            target_position: 0.0,
        }
    }
}

impl ros2_client::Message for StepperState {}
