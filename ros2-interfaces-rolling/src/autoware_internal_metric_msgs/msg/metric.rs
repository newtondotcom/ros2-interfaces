use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    pub name: ::std::string::String,
    pub unit: ::std::string::String,
    pub value: ::std::string::String,
}

impl Default for Metric {
    fn default() -> Self {
        Metric {
            name: ::std::string::String::new(),
            unit: ::std::string::String::new(),
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Metric {}
