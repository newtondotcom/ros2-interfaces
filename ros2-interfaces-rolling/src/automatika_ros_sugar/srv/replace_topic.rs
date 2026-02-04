use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplaceTopicRequest {
    pub direction: u8,
    pub old_name: ::std::string::String,
    pub new_name: ::std::string::String,
    pub new_msg_type: ::std::string::String,
}

impl ReplaceTopicRequest {
    pub const INPUT_TOPIC: u8 = 0;
    pub const OUTPUT_TOPIC: u8 = 1;
}

impl Default for ReplaceTopicRequest {
    fn default() -> Self {
        ReplaceTopicRequest {
            direction: 0,
            old_name: ::std::string::String::new(),
            new_name: ::std::string::String::new(),
            new_msg_type: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ReplaceTopicRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplaceTopicResponse {
    pub success: bool,
    pub error_msg: ::std::string::String,
}

impl Default for ReplaceTopicResponse {
    fn default() -> Self {
        ReplaceTopicResponse {
            success: false,
            error_msg: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ReplaceTopicResponse {}


pub struct ReplaceTopic;
impl ros2_client::Service for ReplaceTopic {
    type Request = ReplaceTopicRequest;
    type Response = ReplaceTopicResponse;

    fn request_type_name(&self) -> &str { "ReplaceTopicRequest" }
    fn response_type_name(&self) -> &str { "ReplaceTopicResponse" }
}
