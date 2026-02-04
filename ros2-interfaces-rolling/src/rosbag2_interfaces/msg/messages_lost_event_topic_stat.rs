use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessagesLostEventTopicStat {
    pub topic_name: ::std::string::String,
    pub messages_lost_in_transport: u64,
    pub messages_lost_in_recorder: u64,
}

impl Default for MessagesLostEventTopicStat {
    fn default() -> Self {
        MessagesLostEventTopicStat {
            topic_name: ::std::string::String::new(),
            messages_lost_in_transport: 0,
            messages_lost_in_recorder: 0,
        }
    }
}

impl ros2_client::Message for MessagesLostEventTopicStat {}
