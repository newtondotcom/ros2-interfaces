use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodesReq {

}

impl Default for NodesReq {
    fn default() -> Self {
        NodesReq {

        }
    }
}

impl ros2_client::Message for NodesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodesRes {
    pub nodes: Vec<::std::string::String>,
}

impl Default for NodesRes {
    fn default() -> Self {
        NodesRes {
            nodes: Vec::new(),
        }
    }
}

impl ros2_client::Message for NodesRes {}


pub struct Nodes;
impl ros2_client::Service for Nodes {
    type Request = NodesReq;
    type Response = NodesRes;

    fn request_type_name(&self) -> &str { "NodesReq" }
    fn response_type_name(&self) -> &str { "NodesRes" }
}
