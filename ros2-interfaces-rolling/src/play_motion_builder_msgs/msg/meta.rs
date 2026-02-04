use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub name: ::std::string::String,
    pub usage: ::std::string::String,
    pub description: ::std::string::String,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            name: ::std::string::String::new(),
            usage: ::std::string::String::new(),
            description: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Meta {}
