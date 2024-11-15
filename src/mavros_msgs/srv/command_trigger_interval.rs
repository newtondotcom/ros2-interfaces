use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTriggerIntervalReq {
    pub cycle_time: f32,
    pub integration_time: f32,
}

impl Default for CommandTriggerIntervalReq {
    fn default() -> Self {
        CommandTriggerIntervalReq {
            cycle_time: 0.0,
            integration_time: 0.0,
        }
    }
}

impl ros2_client::Message for CommandTriggerIntervalReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTriggerIntervalRes {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandTriggerIntervalRes {
    fn default() -> Self {
        CommandTriggerIntervalRes {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandTriggerIntervalRes {}


pub struct CommandTriggerInterval;
impl ros2_client::Service for CommandTriggerInterval {
    type Request = CommandTriggerIntervalReq;
    type Response = CommandTriggerIntervalRes;

    fn request_type_name(&self) -> &str { "CommandTriggerIntervalReq" }
    fn response_type_name(&self) -> &str { "CommandTriggerIntervalRes" }
}
