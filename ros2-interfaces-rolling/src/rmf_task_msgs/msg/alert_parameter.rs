use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertParameter {
    pub name: ::std::string::String,
    pub value: ::std::string::String,
}

impl Default for AlertParameter {
    fn default() -> Self {
        AlertParameter {
            name: ::std::string::String::new(),
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AlertParameter {}
