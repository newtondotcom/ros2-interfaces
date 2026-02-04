use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicStatistic {
    pub statistic_type: u8,
    pub node_name: ::std::string::String,
    pub topic_name: ::std::string::String,
    pub window_count: i32,
    pub mean: crate::builtin_interfaces::msg::Duration,
    pub min: crate::builtin_interfaces::msg::Duration,
    pub max: crate::builtin_interfaces::msg::Duration,
}

impl TopicStatistic {
    pub const PUBLISHED_PERIOD: u8 = 0;
    pub const RECEIVED_PERIOD: u8 = 1;
    pub const TAKE_AGE: u8 = 2;
}

impl Default for TopicStatistic {
    fn default() -> Self {
        TopicStatistic {
            statistic_type: 0,
            node_name: ::std::string::String::new(),
            topic_name: ::std::string::String::new(),
            window_count: 0,
            mean: crate::builtin_interfaces::msg::Duration::default(),
            min: crate::builtin_interfaces::msg::Duration::default(),
            max: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for TopicStatistic {}
