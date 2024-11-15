use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandVtolTransitionReq {
    pub header: crate::std_msgs::msg::Header,
    pub state: u8,
}

impl CommandVtolTransitionReq {
    pub const STATE_MC: u8 = 3;
    pub const STATE_FW: u8 = 4;
}

impl Default for CommandVtolTransitionReq {
    fn default() -> Self {
        CommandVtolTransitionReq {
            header: crate::std_msgs::msg::Header::default(),
            state: 0,
        }
    }
}

impl ros2_client::Message for CommandVtolTransitionReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandVtolTransitionRes {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandVtolTransitionRes {
    fn default() -> Self {
        CommandVtolTransitionRes {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandVtolTransitionRes {}


pub struct CommandVtolTransition;
impl ros2_client::Service for CommandVtolTransition {
    type Request = CommandVtolTransitionReq;
    type Response = CommandVtolTransitionRes;

    fn request_type_name(&self) -> &str { "CommandVtolTransitionReq" }
    fn response_type_name(&self) -> &str { "CommandVtolTransitionRes" }
}
