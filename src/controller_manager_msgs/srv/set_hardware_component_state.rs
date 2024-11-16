use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHardwareComponentStateReq {
    pub name: ::std::string::String,
    pub target_state: crate::lifecycle_msgs::msg::State,
}

impl Default for SetHardwareComponentStateReq {
    fn default() -> Self {
        SetHardwareComponentStateReq {
            name: ::std::string::String::new(),
            target_state: crate::lifecycle_msgs::msg::State::default(),
        }
    }
}

impl ros2_client::Message for SetHardwareComponentStateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHardwareComponentStateRes {
    pub ok: bool,
    pub state: crate::lifecycle_msgs::msg::State,
}

impl Default for SetHardwareComponentStateRes {
    fn default() -> Self {
        SetHardwareComponentStateRes {
            ok: false,
            state: crate::lifecycle_msgs::msg::State::default(),
        }
    }
}

impl ros2_client::Message for SetHardwareComponentStateRes {}


pub struct SetHardwareComponentState;
impl ros2_client::Service for SetHardwareComponentState {
    type Request = SetHardwareComponentStateReq;
    type Response = SetHardwareComponentStateRes;

    fn request_type_name(&self) -> &str { "SetHardwareComponentStateReq" }
    fn response_type_name(&self) -> &str { "SetHardwareComponentStateRes" }
}
