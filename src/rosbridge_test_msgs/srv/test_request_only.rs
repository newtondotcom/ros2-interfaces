use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRequestOnlyReq {
    pub data: i32,
}

impl Default for TestRequestOnlyReq {
    fn default() -> Self {
        TestRequestOnlyReq {
            data: 0,
        }
    }
}

impl ros2_client::Message for TestRequestOnlyReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRequestOnlyRes {

}

impl Default for TestRequestOnlyRes {
    fn default() -> Self {
        TestRequestOnlyRes {

        }
    }
}

impl ros2_client::Message for TestRequestOnlyRes {}


pub struct TestRequestOnly;
impl ros2_client::Service for TestRequestOnly {
    type Request = TestRequestOnlyReq;
    type Response = TestRequestOnlyRes;

    fn request_type_name(&self) -> &str { "TestRequestOnlyReq" }
    fn response_type_name(&self) -> &str { "TestRequestOnlyRes" }
}
