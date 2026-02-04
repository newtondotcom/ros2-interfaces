use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepperConfig {
    pub min_failsafe_time: i32,
    pub max_failsafe_time: i32,
    pub min_position: f64,
    pub max_position: f64,
    pub min_acceleration: f64,
    pub max_acceleration: f64,
    pub min_velocity_limit: f64,
    pub max_velocity_limit: f64,
    pub min_current_limit: f64,
    pub max_current_limit: f64,
    pub min_data_interval: i32,
    pub max_data_interval: i32,
}

impl Default for StepperConfig {
    fn default() -> Self {
        StepperConfig {
            min_failsafe_time: 0,
            max_failsafe_time: 0,
            min_position: 0.0,
            max_position: 0.0,
            min_acceleration: 0.0,
            max_acceleration: 0.0,
            min_velocity_limit: 0.0,
            max_velocity_limit: 0.0,
            min_current_limit: 0.0,
            max_current_limit: 0.0,
            min_data_interval: 0,
            max_data_interval: 0,
        }
    }
}

impl ros2_client::Message for StepperConfig {}
