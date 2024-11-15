use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsReq {

}

impl Default for TopicsReq {
    fn default() -> Self {
        TopicsReq {

        }
    }
}

impl ros2_client::Message for TopicsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsRes {
    pub topics: Vec<::std::string::String>,
    pub types: Vec<::std::string::String>,
}

impl Default for TopicsRes {
    fn default() -> Self {
        TopicsRes {
            topics: Vec::new(),
            types: Vec::new(),
        }
    }
}

impl ros2_client::Message for TopicsRes {}


pub struct Topics;
impl ros2_client::Service for Topics {
    type Request = TopicsReq;
    type Response = TopicsRes;

    fn request_type_name(&self) -> &str { "TopicsReq" }
    fn response_type_name(&self) -> &str { "TopicsRes" }
}
