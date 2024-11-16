use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandLongReq {
    pub broadcast: bool,
    pub command: u16,
    pub confirmation: u8,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub param5: f32,
    pub param6: f32,
    pub param7: f32,
}

impl Default for CommandLongReq {
    fn default() -> Self {
        CommandLongReq {
            broadcast: false,
            command: 0,
            confirmation: 0,
            param1: 0.0,
            param2: 0.0,
            param3: 0.0,
            param4: 0.0,
            param5: 0.0,
            param6: 0.0,
            param7: 0.0,
        }
    }
}

impl ros2_client::Message for CommandLongReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandLongRes {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandLongRes {
    fn default() -> Self {
        CommandLongRes {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandLongRes {}


pub struct CommandLong;
impl ros2_client::Service for CommandLong {
    type Request = CommandLongReq;
    type Response = CommandLongRes;

    fn request_type_name(&self) -> &str { "CommandLongReq" }
    fn response_type_name(&self) -> &str { "CommandLongRes" }
}
