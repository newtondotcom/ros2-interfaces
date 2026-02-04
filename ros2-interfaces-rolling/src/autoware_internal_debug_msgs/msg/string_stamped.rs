use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringStamped {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub data: ::std::string::String,
}

impl Default for StringStamped {
    fn default() -> Self {
        StringStamped {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            data: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StringStamped {}
