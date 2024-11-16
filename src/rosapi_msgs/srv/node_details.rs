use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeDetailsReq {
    pub node: ::std::string::String,
}

impl Default for NodeDetailsReq {
    fn default() -> Self {
        NodeDetailsReq {
            node: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for NodeDetailsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeDetailsRes {
    pub subscribing: Vec<::std::string::String>,
    pub publishing: Vec<::std::string::String>,
    pub services: Vec<::std::string::String>,
}

impl Default for NodeDetailsRes {
    fn default() -> Self {
        NodeDetailsRes {
            subscribing: Vec::new(),
            publishing: Vec::new(),
            services: Vec::new(),
        }
    }
}

impl ros2_client::Message for NodeDetailsRes {}


pub struct NodeDetails;
impl ros2_client::Service for NodeDetails {
    type Request = NodeDetailsReq;
    type Response = NodeDetailsRes;

    fn request_type_name(&self) -> &str { "NodeDetailsReq" }
    fn response_type_name(&self) -> &str { "NodeDetailsRes" }
}
