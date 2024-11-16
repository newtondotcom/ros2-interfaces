use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestHeader {
    pub robot_name: ::std::string::String,
    pub fleet_name: ::std::string::String,
    pub request_id: u64,
}

impl Default for RequestHeader {
    fn default() -> Self {
        RequestHeader {
            robot_name: ::std::string::String::new(),
            fleet_name: ::std::string::String::new(),
            request_id: 0,
        }
    }
}

impl ros2_client::Message for RequestHeader {}
