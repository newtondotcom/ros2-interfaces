use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourcePointCloudInfo {
    pub header: crate::std_msgs::msg::Header,
    pub topic: ::std::string::String,
    pub status: u8,
    pub idx_begin: u32,
    pub length: u32,
}

impl SourcePointCloudInfo {
    pub const STATUS_OK: u8 = 0;
    pub const STATUS_TIMEOUT: u8 = 1;
    pub const STATUS_INVALID: u8 = 2;
}

impl Default for SourcePointCloudInfo {
    fn default() -> Self {
        SourcePointCloudInfo {
            header: crate::std_msgs::msg::Header::default(),
            topic: ::std::string::String::new(),
            status: 0,
            idx_begin: 0,
            length: 0,
        }
    }
}

impl ros2_client::Message for SourcePointCloudInfo {}
