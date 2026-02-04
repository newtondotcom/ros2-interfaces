use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    pub seq: u32,
    pub tier: ::std::string::String,
    pub unix_millis_time: i64,
    pub text: ::std::string::String,
}

impl LogEntry {
    pub const TIER_UNINITIALIZED: &'static str = "uninitialized";
    pub const TIER_INFO: &'static str = "info";
    pub const TIER_WARNING: &'static str = "warning";
    pub const TIER_ERROR: &'static str = "error";
}

impl Default for LogEntry {
    fn default() -> Self {
        LogEntry {
            seq: 0,
            tier: ::std::string::String::new(),
            unix_millis_time: 0,
            text: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LogEntry {}
