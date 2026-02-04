use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceLog {
    pub stamp: crate::builtin_interfaces::msg::Time,
    #[serde(rename = "type")]    pub type_: u8,
    pub name: ::std::string::String,
    pub node: ::std::string::String,
    pub yaml: ::std::string::String,
}

impl ServiceLog {
    pub const CLIENT_REQUEST: u8 = 1;
    pub const SERVER_REQUEST: u8 = 2;
    pub const SERVER_RESPONSE: u8 = 3;
    pub const CLIENT_RESPONSE: u8 = 4;
    pub const ERROR_UNREADY: u8 = 5;
    pub const ERROR_TIMEOUT: u8 = 6;
}

impl Default for ServiceLog {
    fn default() -> Self {
        ServiceLog {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            type_: 0,
            name: ::std::string::String::new(),
            node: ::std::string::String::new(),
            yaml: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServiceLog {}
