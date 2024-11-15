use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMultipleRequestFieldsReq {
    pub int_value: i32,
    pub float_value: f32,
    pub string: ::std::string::String,
    pub bool_value: bool,
}

impl Default for TestMultipleRequestFieldsReq {
    fn default() -> Self {
        TestMultipleRequestFieldsReq {
            int_value: 0,
            float_value: 0.0,
            string: ::std::string::String::new(),
            bool_value: false,
        }
    }
}

impl ros2_client::Message for TestMultipleRequestFieldsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMultipleRequestFieldsRes {

}

impl Default for TestMultipleRequestFieldsRes {
    fn default() -> Self {
        TestMultipleRequestFieldsRes {

        }
    }
}

impl ros2_client::Message for TestMultipleRequestFieldsRes {}


pub struct TestMultipleRequestFields;
impl ros2_client::Service for TestMultipleRequestFields {
    type Request = TestMultipleRequestFieldsReq;
    type Response = TestMultipleRequestFieldsRes;

    fn request_type_name(&self) -> &str { "TestMultipleRequestFieldsReq" }
    fn response_type_name(&self) -> &str { "TestMultipleRequestFieldsRes" }
}
