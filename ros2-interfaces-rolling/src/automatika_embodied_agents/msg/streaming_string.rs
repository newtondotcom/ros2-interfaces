use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamingString {
    pub header: crate::std_msgs::msg::Header,
    pub stream: bool,
    pub done: bool,
    pub data: ::std::string::String,
}

impl Default for StreamingString {
    fn default() -> Self {
        StreamingString {
            header: crate::std_msgs::msg::Header::default(),
            stream: false,
            done: false,
            data: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StreamingString {}
