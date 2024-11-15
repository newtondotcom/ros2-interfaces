use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceNodeReq {
    pub service: ::std::string::String,
}

impl Default for ServiceNodeReq {
    fn default() -> Self {
        ServiceNodeReq {
            service: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServiceNodeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceNodeRes {
    pub node: ::std::string::String,
}

impl Default for ServiceNodeRes {
    fn default() -> Self {
        ServiceNodeRes {
            node: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServiceNodeRes {}


pub struct ServiceNode;
impl ros2_client::Service for ServiceNode {
    type Request = ServiceNodeReq;
    type Response = ServiceNodeRes;

    fn request_type_name(&self) -> &str { "ServiceNodeReq" }
    fn response_type_name(&self) -> &str { "ServiceNodeRes" }
}
