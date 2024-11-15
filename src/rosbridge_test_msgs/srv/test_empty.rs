use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestEmptyReq {

}

impl Default for TestEmptyReq {
    fn default() -> Self {
        TestEmptyReq {

        }
    }
}

impl ros2_client::Message for TestEmptyReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestEmptyRes {

}

impl Default for TestEmptyRes {
    fn default() -> Self {
        TestEmptyRes {

        }
    }
}

impl ros2_client::Message for TestEmptyRes {}


pub struct TestEmpty;
impl ros2_client::Service for TestEmpty {
    type Request = TestEmptyReq;
    type Response = TestEmptyRes;

    fn request_type_name(&self) -> &str { "TestEmptyReq" }
    fn response_type_name(&self) -> &str { "TestEmptyRes" }
}
