use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConcatenatedPointCloudInfo {
    pub header: crate::std_msgs::msg::Header,
    pub concatenation_success: bool,
    pub matching_strategy: u8,
    pub matching_strategy_config: Vec<u8>,
    pub source_info: Vec<crate::autoware_sensing_msgs::msg::SourcePointCloudInfo>,
}

impl ConcatenatedPointCloudInfo {
    pub const STRATEGY_NAIVE: u8 = 0;
    pub const STRATEGY_ADVANCED: u8 = 1;
}

impl Default for ConcatenatedPointCloudInfo {
    fn default() -> Self {
        ConcatenatedPointCloudInfo {
            header: crate::std_msgs::msg::Header::default(),
            concatenation_success: false,
            matching_strategy: 0,
            matching_strategy_config: Vec::new(),
            source_info: Vec::new(),
        }
    }
}

impl ros2_client::Message for ConcatenatedPointCloudInfo {}
