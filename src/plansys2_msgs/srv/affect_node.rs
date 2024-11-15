use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectNodeReq {
    pub node: crate::plansys2_msgs::msg::Node,
}

impl Default for AffectNodeReq {
    fn default() -> Self {
        AffectNodeReq {
            node: crate::plansys2_msgs::msg::Node::default(),
        }
    }
}

impl ros2_client::Message for AffectNodeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectNodeRes {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for AffectNodeRes {
    fn default() -> Self {
        AffectNodeRes {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AffectNodeRes {}


pub struct AffectNode;
impl ros2_client::Service for AffectNode {
    type Request = AffectNodeReq;
    type Response = AffectNodeRes;

    fn request_type_name(&self) -> &str { "AffectNodeReq" }
    fn response_type_name(&self) -> &str { "AffectNodeRes" }
}
