use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepperCommand {
    pub mode: u8,
    pub target: f64,
    pub velocity: f64,
}

impl StepperCommand {
    pub const CONTROL_MODE_DISENGAGED: u8 = 255;
    pub const CONTROL_MODE_STEP: u8 = 0;
    pub const CONTROL_MODE_RUN: u8 = 1;
    pub const CONTROL_MODE_STOP: u8 = 2;
}

impl Default for StepperCommand {
    fn default() -> Self {
        StepperCommand {
            mode: 0,
            target: 0.0,
            velocity: 0.0,
        }
    }
}

impl ros2_client::Message for StepperCommand {}
