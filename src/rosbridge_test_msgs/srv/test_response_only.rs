use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestResponseOnlyReq {

}

impl Default for TestResponseOnlyReq {
    fn default() -> Self {
        TestResponseOnlyReq {

        }
    }
}

impl ros2_client::Message for TestResponseOnlyReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestResponseOnlyRes {
    pub data: i32,
}

impl Default for TestResponseOnlyRes {
    fn default() -> Self {
        TestResponseOnlyRes {
            data: 0,
        }
    }
}

impl ros2_client::Message for TestResponseOnlyRes {}


pub struct TestResponseOnly;
impl ros2_client::Service for TestResponseOnly {
    type Request = TestResponseOnlyReq;
    type Response = TestResponseOnlyRes;

    fn request_type_name(&self) -> &str { "TestResponseOnlyReq" }
    fn response_type_name(&self) -> &str { "TestResponseOnlyRes" }
}
