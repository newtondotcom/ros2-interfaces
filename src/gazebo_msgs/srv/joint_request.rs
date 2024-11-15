use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointRequestReq {
    pub joint_name: ::std::string::String,
}

impl Default for JointRequestReq {
    fn default() -> Self {
        JointRequestReq {
            joint_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for JointRequestReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointRequestRes {

}

impl Default for JointRequestRes {
    fn default() -> Self {
        JointRequestRes {

        }
    }
}

impl ros2_client::Message for JointRequestRes {}


pub struct JointRequest;
impl ros2_client::Service for JointRequest {
    type Request = JointRequestReq;
    type Response = JointRequestRes;

    fn request_type_name(&self) -> &str { "JointRequestReq" }
    fn response_type_name(&self) -> &str { "JointRequestRes" }
}
