use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLinkStateReq {
    pub link_state: crate::gazebo_msgs::msg::LinkState,
}

impl Default for SetLinkStateReq {
    fn default() -> Self {
        SetLinkStateReq {
            link_state: crate::gazebo_msgs::msg::LinkState::default(),
        }
    }
}

impl ros2_client::Message for SetLinkStateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLinkStateRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetLinkStateRes {
    fn default() -> Self {
        SetLinkStateRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetLinkStateRes {}


pub struct SetLinkState;
impl ros2_client::Service for SetLinkState {
    type Request = SetLinkStateReq;
    type Response = SetLinkStateRes;

    fn request_type_name(&self) -> &str { "SetLinkStateReq" }
    fn response_type_name(&self) -> &str { "SetLinkStateRes" }
}
