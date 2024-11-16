use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMultipleResponseFieldsReq {

}

impl Default for TestMultipleResponseFieldsReq {
    fn default() -> Self {
        TestMultipleResponseFieldsReq {

        }
    }
}

impl ros2_client::Message for TestMultipleResponseFieldsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMultipleResponseFieldsRes {
    pub int_value: i32,
    pub float_value: f32,
    pub string: ::std::string::String,
    pub bool_value: bool,
}

impl Default for TestMultipleResponseFieldsRes {
    fn default() -> Self {
        TestMultipleResponseFieldsRes {
            int_value: 0,
            float_value: 0.0,
            string: ::std::string::String::new(),
            bool_value: false,
        }
    }
}

impl ros2_client::Message for TestMultipleResponseFieldsRes {}


pub struct TestMultipleResponseFields;
impl ros2_client::Service for TestMultipleResponseFields {
    type Request = TestMultipleResponseFieldsReq;
    type Response = TestMultipleResponseFieldsRes;

    fn request_type_name(&self) -> &str { "TestMultipleResponseFieldsReq" }
    fn response_type_name(&self) -> &str { "TestMultipleResponseFieldsRes" }
}
