use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandBoolReq {
    pub value: bool,
}

impl Default for CommandBoolReq {
    fn default() -> Self {
        CommandBoolReq {
            value: false,
        }
    }
}

impl ros2_client::Message for CommandBoolReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandBoolRes {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandBoolRes {
    fn default() -> Self {
        CommandBoolRes {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandBoolRes {}


pub struct CommandBool;
impl ros2_client::Service for CommandBool {
    type Request = CommandBoolReq;
    type Response = CommandBoolRes;

    fn request_type_name(&self) -> &str { "CommandBoolReq" }
    fn response_type_name(&self) -> &str { "CommandBoolRes" }
}
