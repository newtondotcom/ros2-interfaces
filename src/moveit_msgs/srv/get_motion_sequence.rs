use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMotionSequenceReq {
    pub request: crate::moveit_msgs::msg::MotionSequenceRequest,
}

impl Default for GetMotionSequenceReq {
    fn default() -> Self {
        GetMotionSequenceReq {
            request: crate::moveit_msgs::msg::MotionSequenceRequest::default(),
        }
    }
}

impl ros2_client::Message for GetMotionSequenceReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMotionSequenceRes {
    pub response: crate::moveit_msgs::msg::MotionSequenceResponse,
}

impl Default for GetMotionSequenceRes {
    fn default() -> Self {
        GetMotionSequenceRes {
            response: crate::moveit_msgs::msg::MotionSequenceResponse::default(),
        }
    }
}

impl ros2_client::Message for GetMotionSequenceRes {}


pub struct GetMotionSequence;
impl ros2_client::Service for GetMotionSequence {
    type Request = GetMotionSequenceReq;
    type Response = GetMotionSequenceRes;

    fn request_type_name(&self) -> &str { "GetMotionSequenceReq" }
    fn response_type_name(&self) -> &str { "GetMotionSequenceRes" }
}
