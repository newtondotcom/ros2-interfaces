use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetModelStateReq {
    pub model_state: crate::gazebo_msgs::msg::ModelState,
}

impl Default for SetModelStateReq {
    fn default() -> Self {
        SetModelStateReq {
            model_state: crate::gazebo_msgs::msg::ModelState::default(),
        }
    }
}

impl ros2_client::Message for SetModelStateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetModelStateRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetModelStateRes {
    fn default() -> Self {
        SetModelStateRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetModelStateRes {}


pub struct SetModelState;
impl ros2_client::Service for SetModelState {
    type Request = SetModelStateReq;
    type Response = SetModelStateRes;

    fn request_type_name(&self) -> &str { "SetModelStateReq" }
    fn response_type_name(&self) -> &str { "SetModelStateRes" }
}
