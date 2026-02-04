use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessagesLostEvent {
    pub node_name: ::std::string::String,
    pub messages_lost_statistics: Vec<crate::rosbag2_interfaces::msg::MessagesLostEventTopicStat>,
}

impl Default for MessagesLostEvent {
    fn default() -> Self {
        MessagesLostEvent {
            node_name: ::std::string::String::new(),
            messages_lost_statistics: Vec::new(),
        }
    }
}

impl ros2_client::Message for MessagesLostEvent {}
