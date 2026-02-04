use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicStatistics {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub statistics: Vec<crate::rosgraph_monitor_msgs::msg::TopicStatistic>,
}

impl Default for TopicStatistics {
    fn default() -> Self {
        TopicStatistics {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            statistics: Vec::new(),
        }
    }
}

impl ros2_client::Message for TopicStatistics {}
