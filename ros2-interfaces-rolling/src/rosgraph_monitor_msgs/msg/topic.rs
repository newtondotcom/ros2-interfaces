use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    pub name: ::std::string::String,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub qos: crate::rosgraph_monitor_msgs::msg::QosProfile,
}

impl Default for Topic {
    fn default() -> Self {
        Topic {
            name: ::std::string::String::new(),
            type_: ::std::string::String::new(),
            qos: crate::rosgraph_monitor_msgs::msg::QosProfile::default(),
        }
    }
}

impl ros2_client::Message for Topic {}
