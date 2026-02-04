use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    pub id: ::std::string::String,
    pub title: ::std::string::String,
    pub subtitle: ::std::string::String,
    pub message: ::std::string::String,
    pub display: bool, // default: true
    pub tier: u8,
    pub responses_available: Vec<::std::string::String>,
    pub alert_parameters: Vec<crate::rmf_task_msgs::msg::AlertParameter>,
    pub task_id: ::std::string::String,
}

impl Alert {
    pub const TIER_INFO: u8 = 0;
    pub const TIER_WARNING: u8 = 1;
    pub const TIER_ERROR: u8 = 2;
}

impl Default for Alert {
    fn default() -> Self {
        Alert {
            id: ::std::string::String::new(),
            title: ::std::string::String::new(),
            subtitle: ::std::string::String::new(),
            message: ::std::string::String::new(),
            display: true,
            tier: 0,
            responses_available: Vec::new(),
            alert_parameters: Vec::new(),
            task_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Alert {}
