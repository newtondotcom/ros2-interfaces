use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicTypeReq {
    pub topic: ::std::string::String,
}

impl Default for TopicTypeReq {
    fn default() -> Self {
        TopicTypeReq {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for TopicTypeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicTypeRes {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for TopicTypeRes {
    fn default() -> Self {
        TopicTypeRes {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for TopicTypeRes {}


pub struct TopicType;
impl ros2_client::Service for TopicType {
    type Request = TopicTypeReq;
    type Response = TopicTypeRes;

    fn request_type_name(&self) -> &str { "TopicTypeReq" }
    fn response_type_name(&self) -> &str { "TopicTypeRes" }
}
