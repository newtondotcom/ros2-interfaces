use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VelocityLimitClearCommand {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub command: bool, // default: false
    pub sender: ::std::string::String,
}

impl Default for VelocityLimitClearCommand {
    fn default() -> Self {
        VelocityLimitClearCommand {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            command: false,
            sender: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for VelocityLimitClearCommand {}
