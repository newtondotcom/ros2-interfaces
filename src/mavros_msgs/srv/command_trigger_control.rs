use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTriggerControlReq {
    pub trigger_enable: bool,
    pub sequence_reset: bool,
    pub trigger_pause: bool,
}

impl Default for CommandTriggerControlReq {
    fn default() -> Self {
        CommandTriggerControlReq {
            trigger_enable: false,
            sequence_reset: false,
            trigger_pause: false,
        }
    }
}

impl ros2_client::Message for CommandTriggerControlReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTriggerControlRes {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandTriggerControlRes {
    fn default() -> Self {
        CommandTriggerControlRes {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandTriggerControlRes {}


pub struct CommandTriggerControl;
impl ros2_client::Service for CommandTriggerControl {
    type Request = CommandTriggerControlReq;
    type Response = CommandTriggerControlRes;

    fn request_type_name(&self) -> &str { "CommandTriggerControlReq" }
    fn response_type_name(&self) -> &str { "CommandTriggerControlRes" }
}
