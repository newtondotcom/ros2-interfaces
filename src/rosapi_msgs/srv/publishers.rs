use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishersReq {
    pub topic: ::std::string::String,
}

impl Default for PublishersReq {
    fn default() -> Self {
        PublishersReq {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for PublishersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishersRes {
    pub publishers: Vec<::std::string::String>,
}

impl Default for PublishersRes {
    fn default() -> Self {
        PublishersRes {
            publishers: Vec::new(),
        }
    }
}

impl ros2_client::Message for PublishersRes {}


pub struct Publishers;
impl ros2_client::Service for Publishers {
    type Request = PublishersReq;
    type Response = PublishersRes;

    fn request_type_name(&self) -> &str { "PublishersReq" }
    fn response_type_name(&self) -> &str { "PublishersRes" }
}
