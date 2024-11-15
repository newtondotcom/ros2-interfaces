use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandHomeReq {
    pub current_gps: bool,
    pub yaw: f32,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
}

impl Default for CommandHomeReq {
    fn default() -> Self {
        CommandHomeReq {
            current_gps: false,
            yaw: 0.0,
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
        }
    }
}

impl ros2_client::Message for CommandHomeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandHomeRes {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandHomeRes {
    fn default() -> Self {
        CommandHomeRes {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandHomeRes {}


pub struct CommandHome;
impl ros2_client::Service for CommandHome {
    type Request = CommandHomeReq;
    type Response = CommandHomeRes;

    fn request_type_name(&self) -> &str { "CommandHomeReq" }
    fn response_type_name(&self) -> &str { "CommandHomeRes" }
}
