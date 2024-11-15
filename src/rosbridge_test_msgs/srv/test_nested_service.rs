use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestNestedServiceReq {
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for TestNestedServiceReq {
    fn default() -> Self {
        TestNestedServiceReq {
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for TestNestedServiceReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestNestedServiceRes {
    pub data: crate::std_msgs::msg::Float64,
}

impl Default for TestNestedServiceRes {
    fn default() -> Self {
        TestNestedServiceRes {
            data: crate::std_msgs::msg::Float64::default(),
        }
    }
}

impl ros2_client::Message for TestNestedServiceRes {}


pub struct TestNestedService;
impl ros2_client::Service for TestNestedService {
    type Request = TestNestedServiceReq;
    type Response = TestNestedServiceRes;

    fn request_type_name(&self) -> &str { "TestNestedServiceReq" }
    fn response_type_name(&self) -> &str { "TestNestedServiceRes" }
}
