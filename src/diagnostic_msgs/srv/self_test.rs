use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfTestReq {

}

impl Default for SelfTestReq {
    fn default() -> Self {
        SelfTestReq {

        }
    }
}

impl ros2_client::Message for SelfTestReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfTestRes {
    pub id: ::std::string::String,
    pub passed: u8,
    pub status: Vec<crate::diagnostic_msgs::msg::DiagnosticStatus>,
}

impl Default for SelfTestRes {
    fn default() -> Self {
        SelfTestRes {
            id: ::std::string::String::new(),
            passed: 0,
            status: Vec::new(),
        }
    }
}

impl ros2_client::Message for SelfTestRes {}


pub struct SelfTest;
impl ros2_client::Service for SelfTest {
    type Request = SelfTestReq;
    type Response = SelfTestRes;

    fn request_type_name(&self) -> &str { "SelfTestReq" }
    fn response_type_name(&self) -> &str { "SelfTestRes" }
}
