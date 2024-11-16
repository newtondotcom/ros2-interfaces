use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCMGraphReq {
    pub node_ids: Vec<u64>,
}

impl Default for GetCMGraphReq {
    fn default() -> Self {
        GetCMGraphReq {
            node_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetCMGraphReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCMGraphRes {
    pub cm_graph: crate::mrpt_msgs::msg::NetworkOfPoses,
}

impl Default for GetCMGraphRes {
    fn default() -> Self {
        GetCMGraphRes {
            cm_graph: crate::mrpt_msgs::msg::NetworkOfPoses::default(),
        }
    }
}

impl ros2_client::Message for GetCMGraphRes {}


pub struct GetCMGraph;
impl ros2_client::Service for GetCMGraph {
    type Request = GetCMGraphReq;
    type Response = GetCMGraphRes;

    fn request_type_name(&self) -> &str { "GetCMGraphReq" }
    fn response_type_name(&self) -> &str { "GetCMGraphRes" }
}
