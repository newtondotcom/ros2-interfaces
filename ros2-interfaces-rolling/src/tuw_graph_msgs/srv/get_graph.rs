use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGraphRequest {

}

impl Default for GetGraphRequest {
    fn default() -> Self {
        GetGraphRequest {

        }
    }
}

impl ros2_client::Message for GetGraphRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGraphResponse {
    pub graph: crate::tuw_graph_msgs::msg::Graph,
}

impl Default for GetGraphResponse {
    fn default() -> Self {
        GetGraphResponse {
            graph: crate::tuw_graph_msgs::msg::Graph::default(),
        }
    }
}

impl ros2_client::Message for GetGraphResponse {}


pub struct GetGraph;
impl ros2_client::Service for GetGraph {
    type Request = GetGraphRequest;
    type Response = GetGraphResponse;

    fn request_type_name(&self) -> &str { "GetGraphRequest" }
    fn response_type_name(&self) -> &str { "GetGraphResponse" }
}
