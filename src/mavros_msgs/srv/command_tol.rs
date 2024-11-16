use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTOLReq {
    pub min_pitch: f32,
    pub yaw: f32,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
}

impl Default for CommandTOLReq {
    fn default() -> Self {
        CommandTOLReq {
            min_pitch: 0.0,
            yaw: 0.0,
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
        }
    }
}

impl ros2_client::Message for CommandTOLReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTOLRes {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandTOLRes {
    fn default() -> Self {
        CommandTOLRes {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandTOLRes {}


pub struct CommandTOL;
impl ros2_client::Service for CommandTOL {
    type Request = CommandTOLReq;
    type Response = CommandTOLRes;

    fn request_type_name(&self) -> &str { "CommandTOLReq" }
    fn response_type_name(&self) -> &str { "CommandTOLRes" }
}
