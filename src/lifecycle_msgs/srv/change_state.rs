use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeStateReq {
    pub transition: crate::lifecycle_msgs::msg::Transition,
}

impl Default for ChangeStateReq {
    fn default() -> Self {
        ChangeStateReq {
            transition: crate::lifecycle_msgs::msg::Transition::default(),
        }
    }
}

impl ros2_client::Message for ChangeStateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeStateRes {
    pub success: bool,
}

impl Default for ChangeStateRes {
    fn default() -> Self {
        ChangeStateRes {
            success: false,
        }
    }
}

impl ros2_client::Message for ChangeStateRes {}


pub struct ChangeState;
impl ros2_client::Service for ChangeState {
    type Request = ChangeStateReq;
    type Response = ChangeStateRes;

    fn request_type_name(&self) -> &str { "ChangeStateReq" }
    fn response_type_name(&self) -> &str { "ChangeStateRes" }
}
