use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListNodesReq {

}

impl Default for ListNodesReq {
    fn default() -> Self {
        ListNodesReq {

        }
    }
}

impl ros2_client::Message for ListNodesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListNodesRes {
    pub full_node_names: Vec<::std::string::String>,
    pub unique_ids: Vec<u64>,
}

impl Default for ListNodesRes {
    fn default() -> Self {
        ListNodesRes {
            full_node_names: Vec::new(),
            unique_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListNodesRes {}


pub struct ListNodes;
impl ros2_client::Service for ListNodes {
    type Request = ListNodesReq;
    type Response = ListNodesRes;

    fn request_type_name(&self) -> &str { "ListNodesReq" }
    fn response_type_name(&self) -> &str { "ListNodesRes" }
}
