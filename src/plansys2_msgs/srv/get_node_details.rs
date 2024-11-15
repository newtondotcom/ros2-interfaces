use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeDetailsReq {
    pub expression: ::std::string::String,
}

impl Default for GetNodeDetailsReq {
    fn default() -> Self {
        GetNodeDetailsReq {
            expression: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetNodeDetailsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeDetailsRes {
    pub success: bool,
    pub node: crate::plansys2_msgs::msg::Node,
    pub error_info: ::std::string::String,
}

impl Default for GetNodeDetailsRes {
    fn default() -> Self {
        GetNodeDetailsRes {
            success: false,
            node: crate::plansys2_msgs::msg::Node::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetNodeDetailsRes {}


pub struct GetNodeDetails;
impl ros2_client::Service for GetNodeDetails {
    type Request = GetNodeDetailsReq;
    type Response = GetNodeDetailsRes;

    fn request_type_name(&self) -> &str { "GetNodeDetailsReq" }
    fn response_type_name(&self) -> &str { "GetNodeDetailsRes" }
}
