use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    pub uri: ::std::string::String,
    pub resource_string: ::std::string::String,
}

impl Default for Resource {
    fn default() -> Self {
        Resource {
            uri: ::std::string::String::new(),
            resource_string: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Resource {}
