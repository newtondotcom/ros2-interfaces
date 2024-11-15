use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExistNodeReq {
    pub node: crate::plansys2_msgs::msg::Node,
}

impl Default for ExistNodeReq {
    fn default() -> Self {
        ExistNodeReq {
            node: crate::plansys2_msgs::msg::Node::default(),
        }
    }
}

impl ros2_client::Message for ExistNodeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExistNodeRes {
    pub exist: bool,
}

impl Default for ExistNodeRes {
    fn default() -> Self {
        ExistNodeRes {
            exist: false,
        }
    }
}

impl ros2_client::Message for ExistNodeRes {}


pub struct ExistNode;
impl ros2_client::Service for ExistNode {
    type Request = ExistNodeReq;
    type Response = ExistNodeRes;

    fn request_type_name(&self) -> &str { "ExistNodeReq" }
    fn response_type_name(&self) -> &str { "ExistNodeRes" }
}
