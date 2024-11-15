use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrospectSubscribersReq {

}

impl Default for IntrospectSubscribersReq {
    fn default() -> Self {
        IntrospectSubscribersReq {

        }
    }
}

impl ros2_client::Message for IntrospectSubscribersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrospectSubscribersRes {
    pub subscriber_details: Vec<crate::py_trees_ros_interfaces::msg::SubscriberDetails>,
}

impl Default for IntrospectSubscribersRes {
    fn default() -> Self {
        IntrospectSubscribersRes {
            subscriber_details: Vec::new(),
        }
    }
}

impl ros2_client::Message for IntrospectSubscribersRes {}


pub struct IntrospectSubscribers;
impl ros2_client::Service for IntrospectSubscribers {
    type Request = IntrospectSubscribersReq;
    type Response = IntrospectSubscribersRes;

    fn request_type_name(&self) -> &str { "IntrospectSubscribersReq" }
    fn response_type_name(&self) -> &str { "IntrospectSubscribersRes" }
}
