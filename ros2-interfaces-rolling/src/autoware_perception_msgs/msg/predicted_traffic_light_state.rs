use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictedTrafficLightState {
    pub predicted_stamp: crate::builtin_interfaces::msg::Time,
    pub simultaneous_elements: Vec<crate::autoware_perception_msgs::msg::TrafficLightElement>,
    pub reliability: f32,
    pub information_source: ::std::string::String,
}

impl PredictedTrafficLightState {
    pub const INFORMATION_SOURCE_V2N: &'static str = "V2N";
    pub const INFORMATION_SOURCE_V2I: &'static str = "V2I";
    pub const INFORMATION_SOURCE_V2V: &'static str = "V2V";
    pub const INFORMATION_SOURCE_LOCAL_PERCEPTION: &'static str = "LOCAL_PERCEPTION";
    pub const INFORMATION_SOURCE_MANUAL_OVERRIDE: &'static str = "MANUAL_OVERRIDE";
    pub const INFORMATION_SOURCE_SIMULATION: &'static str = "SIMULATION";
    pub const INFORMATION_SOURCE_INTERNAL_ESTIMATION: &'static str = "INTERNAL_ESTIMATION";
}

impl Default for PredictedTrafficLightState {
    fn default() -> Self {
        PredictedTrafficLightState {
            predicted_stamp: crate::builtin_interfaces::msg::Time::default(),
            simultaneous_elements: Vec::new(),
            reliability: 0.0,
            information_source: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for PredictedTrafficLightState {}
