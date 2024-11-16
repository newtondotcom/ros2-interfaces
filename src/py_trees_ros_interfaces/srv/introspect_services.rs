use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrospectServicesReq {

}

impl Default for IntrospectServicesReq {
    fn default() -> Self {
        IntrospectServicesReq {

        }
    }
}

impl ros2_client::Message for IntrospectServicesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrospectServicesRes {
    pub service_details: Vec<crate::py_trees_ros_interfaces::msg::ServiceDetails>,
}

impl Default for IntrospectServicesRes {
    fn default() -> Self {
        IntrospectServicesRes {
            service_details: Vec::new(),
        }
    }
}

impl ros2_client::Message for IntrospectServicesRes {}


pub struct IntrospectServices;
impl ros2_client::Service for IntrospectServices {
    type Request = IntrospectServicesReq;
    type Response = IntrospectServicesRes;

    fn request_type_name(&self) -> &str { "IntrospectServicesReq" }
    fn response_type_name(&self) -> &str { "IntrospectServicesRes" }
}
