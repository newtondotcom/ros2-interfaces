use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsForTypeReq {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for TopicsForTypeReq {
    fn default() -> Self {
        TopicsForTypeReq {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for TopicsForTypeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsForTypeRes {
    pub topics: Vec<::std::string::String>,
}

impl Default for TopicsForTypeRes {
    fn default() -> Self {
        TopicsForTypeRes {
            topics: Vec::new(),
        }
    }
}

impl ros2_client::Message for TopicsForTypeRes {}


pub struct TopicsForType;
impl ros2_client::Service for TopicsForType {
    type Request = TopicsForTypeReq;
    type Response = TopicsForTypeRes;

    fn request_type_name(&self) -> &str { "TopicsForTypeReq" }
    fn response_type_name(&self) -> &str { "TopicsForTypeRes" }
}
