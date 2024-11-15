use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGraphReq {
    pub graph: crate::fuse_msgs::msg::SerializedGraph,
}

impl Default for SetGraphReq {
    fn default() -> Self {
        SetGraphReq {
            graph: crate::fuse_msgs::msg::SerializedGraph::default(),
        }
    }
}

impl ros2_client::Message for SetGraphReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGraphRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetGraphRes {
    fn default() -> Self {
        SetGraphRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetGraphRes {}


pub struct SetGraph;
impl ros2_client::Service for SetGraph {
    type Request = SetGraphReq;
    type Response = SetGraphRes;

    fn request_type_name(&self) -> &str { "SetGraphReq" }
    fn response_type_name(&self) -> &str { "SetGraphRes" }
}
