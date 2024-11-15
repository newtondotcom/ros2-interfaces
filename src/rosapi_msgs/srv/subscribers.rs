use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribersReq {
    pub topic: ::std::string::String,
}

impl Default for SubscribersReq {
    fn default() -> Self {
        SubscribersReq {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SubscribersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribersRes {
    pub subscribers: Vec<::std::string::String>,
}

impl Default for SubscribersRes {
    fn default() -> Self {
        SubscribersRes {
            subscribers: Vec::new(),
        }
    }
}

impl ros2_client::Message for SubscribersRes {}


pub struct Subscribers;
impl ros2_client::Service for Subscribers {
    type Request = SubscribersReq;
    type Response = SubscribersRes;

    fn request_type_name(&self) -> &str { "SubscribersReq" }
    fn response_type_name(&self) -> &str { "SubscribersRes" }
}
