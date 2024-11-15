use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageDetailsReq {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for MessageDetailsReq {
    fn default() -> Self {
        MessageDetailsReq {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MessageDetailsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageDetailsRes {
    pub typedefs: Vec<crate::rosapi_msgs::msg::TypeDef>,
}

impl Default for MessageDetailsRes {
    fn default() -> Self {
        MessageDetailsRes {
            typedefs: Vec::new(),
        }
    }
}

impl ros2_client::Message for MessageDetailsRes {}


pub struct MessageDetails;
impl ros2_client::Service for MessageDetails {
    type Request = MessageDetailsReq;
    type Response = MessageDetailsRes;

    fn request_type_name(&self) -> &str { "MessageDetailsReq" }
    fn response_type_name(&self) -> &str { "MessageDetailsRes" }
}
