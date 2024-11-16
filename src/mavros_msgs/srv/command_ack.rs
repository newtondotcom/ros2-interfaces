use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandAckReq {
    pub command: u16,
    pub result: u8,
    pub progress: u8,
    pub result_param2: u32,
}

impl Default for CommandAckReq {
    fn default() -> Self {
        CommandAckReq {
            command: 0,
            result: 0,
            progress: 0,
            result_param2: 0,
        }
    }
}

impl ros2_client::Message for CommandAckReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandAckRes {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandAckRes {
    fn default() -> Self {
        CommandAckRes {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandAckRes {}


pub struct CommandAck;
impl ros2_client::Service for CommandAck {
    type Request = CommandAckReq;
    type Response = CommandAckRes;

    fn request_type_name(&self) -> &str { "CommandAckReq" }
    fn response_type_name(&self) -> &str { "CommandAckRes" }
}
