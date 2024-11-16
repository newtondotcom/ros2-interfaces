use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRequestAndResponseReq {
    pub data: i32,
}

impl Default for TestRequestAndResponseReq {
    fn default() -> Self {
        TestRequestAndResponseReq {
            data: 0,
        }
    }
}

impl ros2_client::Message for TestRequestAndResponseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRequestAndResponseRes {
    pub data: i32,
}

impl Default for TestRequestAndResponseRes {
    fn default() -> Self {
        TestRequestAndResponseRes {
            data: 0,
        }
    }
}

impl ros2_client::Message for TestRequestAndResponseRes {}


pub struct TestRequestAndResponse;
impl ros2_client::Service for TestRequestAndResponse {
    type Request = TestRequestAndResponseReq;
    type Response = TestRequestAndResponseRes;

    fn request_type_name(&self) -> &str { "TestRequestAndResponseReq" }
    fn response_type_name(&self) -> &str { "TestRequestAndResponseRes" }
}
