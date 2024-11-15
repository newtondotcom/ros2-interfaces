use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandIntReq {
    pub broadcast: bool,
    pub frame: u8,
    pub command: u16,
    pub current: u8,
    pub autocontinue: u8,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub x: i32,
    pub y: i32,
    pub z: f32,
}

impl Default for CommandIntReq {
    fn default() -> Self {
        CommandIntReq {
            broadcast: false,
            frame: 0,
            command: 0,
            current: 0,
            autocontinue: 0,
            param1: 0.0,
            param2: 0.0,
            param3: 0.0,
            param4: 0.0,
            x: 0,
            y: 0,
            z: 0.0,
        }
    }
}

impl ros2_client::Message for CommandIntReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandIntRes {
    pub success: bool,
}

impl Default for CommandIntRes {
    fn default() -> Self {
        CommandIntRes {
            success: false,
        }
    }
}

impl ros2_client::Message for CommandIntRes {}


pub struct CommandInt;
impl ros2_client::Service for CommandInt {
    type Request = CommandIntReq;
    type Response = CommandIntRes;

    fn request_type_name(&self) -> &str { "CommandIntReq" }
    fn response_type_name(&self) -> &str { "CommandIntRes" }
}
