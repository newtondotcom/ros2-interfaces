use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestArrayRequestReq {
    pub int_values: Vec<i32>,
}

impl Default for TestArrayRequestReq {
    fn default() -> Self {
        TestArrayRequestReq {
            int_values: Vec::new(),
        }
    }
}

impl ros2_client::Message for TestArrayRequestReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestArrayRequestRes {

}

impl Default for TestArrayRequestRes {
    fn default() -> Self {
        TestArrayRequestRes {

        }
    }
}

impl ros2_client::Message for TestArrayRequestRes {}


pub struct TestArrayRequest;
impl ros2_client::Service for TestArrayRequest {
    type Request = TestArrayRequestReq;
    type Response = TestArrayRequestRes;

    fn request_type_name(&self) -> &str { "TestArrayRequestReq" }
    fn response_type_name(&self) -> &str { "TestArrayRequestRes" }
}
