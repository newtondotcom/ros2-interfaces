use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrospectPublishersReq {

}

impl Default for IntrospectPublishersReq {
    fn default() -> Self {
        IntrospectPublishersReq {

        }
    }
}

impl ros2_client::Message for IntrospectPublishersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrospectPublishersRes {
    pub publisher_details: Vec<crate::py_trees_ros_interfaces::msg::PublisherDetails>,
}

impl Default for IntrospectPublishersRes {
    fn default() -> Self {
        IntrospectPublishersRes {
            publisher_details: Vec::new(),
        }
    }
}

impl ros2_client::Message for IntrospectPublishersRes {}


pub struct IntrospectPublishers;
impl ros2_client::Service for IntrospectPublishers {
    type Request = IntrospectPublishersReq;
    type Response = IntrospectPublishersRes;

    fn request_type_name(&self) -> &str { "IntrospectPublishersReq" }
    fn response_type_name(&self) -> &str { "IntrospectPublishersRes" }
}
