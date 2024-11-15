use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MicroROSSelfTestReq {

}

impl Default for MicroROSSelfTestReq {
    fn default() -> Self {
        MicroROSSelfTestReq {

        }
    }
}

impl ros2_client::Message for MicroROSSelfTestReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MicroROSSelfTestRes {
    pub id: ::std::string::String,
    pub passed: u8,
    pub status: crate::micro_ros_diagnostic_msgs::msg::MicroROSDiagnosticStatus,
}

impl Default for MicroROSSelfTestRes {
    fn default() -> Self {
        MicroROSSelfTestRes {
            id: ::std::string::String::new(),
            passed: 0,
            status: crate::micro_ros_diagnostic_msgs::msg::MicroROSDiagnosticStatus::default(),
        }
    }
}

impl ros2_client::Message for MicroROSSelfTestRes {}


pub struct MicroROSSelfTest;
impl ros2_client::Service for MicroROSSelfTest {
    type Request = MicroROSSelfTestReq;
    type Response = MicroROSSelfTestRes;

    fn request_type_name(&self) -> &str { "MicroROSSelfTestReq" }
    fn response_type_name(&self) -> &str { "MicroROSSelfTestRes" }
}
