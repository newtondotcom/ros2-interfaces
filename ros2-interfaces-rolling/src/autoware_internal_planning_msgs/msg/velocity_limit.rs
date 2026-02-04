use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VelocityLimit {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub max_velocity: f32,
    pub use_constraints: bool, // default: false
    pub constraints: crate::autoware_internal_planning_msgs::msg::VelocityLimitConstraints,
    pub sender: ::std::string::String,
}

impl VelocityLimit {
    pub const SENDER_API: &'static str = "api";
}

impl Default for VelocityLimit {
    fn default() -> Self {
        VelocityLimit {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            max_velocity: 0.0,
            use_constraints: false,
            constraints: crate::autoware_internal_planning_msgs::msg::VelocityLimitConstraints::default(),
            sender: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for VelocityLimit {}
