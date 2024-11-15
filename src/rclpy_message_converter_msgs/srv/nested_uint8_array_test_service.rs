use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NestedUint8ArrayTestServiceReq {
    pub input: crate::rclpy_message_converter_msgs::msg::NestedUint8ArrayTestMessage,
}

impl Default for NestedUint8ArrayTestServiceReq {
    fn default() -> Self {
        NestedUint8ArrayTestServiceReq {
            input: crate::rclpy_message_converter_msgs::msg::NestedUint8ArrayTestMessage::default(),
        }
    }
}

impl ros2_client::Message for NestedUint8ArrayTestServiceReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NestedUint8ArrayTestServiceRes {
    pub output: crate::rclpy_message_converter_msgs::msg::NestedUint8ArrayTestMessage,
}

impl Default for NestedUint8ArrayTestServiceRes {
    fn default() -> Self {
        NestedUint8ArrayTestServiceRes {
            output: crate::rclpy_message_converter_msgs::msg::NestedUint8ArrayTestMessage::default(),
        }
    }
}

impl ros2_client::Message for NestedUint8ArrayTestServiceRes {}


pub struct NestedUint8ArrayTestService;
impl ros2_client::Service for NestedUint8ArrayTestService {
    type Request = NestedUint8ArrayTestServiceReq;
    type Response = NestedUint8ArrayTestServiceRes;

    fn request_type_name(&self) -> &str { "NestedUint8ArrayTestServiceReq" }
    fn response_type_name(&self) -> &str { "NestedUint8ArrayTestServiceRes" }
}
