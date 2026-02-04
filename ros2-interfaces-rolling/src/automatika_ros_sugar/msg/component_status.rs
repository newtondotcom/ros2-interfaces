use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentStatus {
    pub status: u8,
    pub msg: ::std::string::String,
    pub src_topics: Vec<::std::string::String>,
    pub src_components: Vec<::std::string::String>,
    pub src_algorithms: Vec<::std::string::String>,
}

impl ComponentStatus {
    pub const STATUS_HEALTHY: u8 = 0;
    pub const STATUS_FAILURE_ALGORITHM_LEVEL: u8 = 1;
    pub const STATUS_FAILURE_COMPONENT_LEVEL: u8 = 2;
    pub const STATUS_FAILURE_SYSTEM_LEVEL: u8 = 3;
    pub const STATUS_GENERAL_FAILURE: u8 = 4;
}

impl Default for ComponentStatus {
    fn default() -> Self {
        ComponentStatus {
            status: 0,
            msg: ::std::string::String::new(),
            src_topics: Vec::new(),
            src_components: Vec::new(),
            src_algorithms: Vec::new(),
        }
    }
}

impl ros2_client::Message for ComponentStatus {}
