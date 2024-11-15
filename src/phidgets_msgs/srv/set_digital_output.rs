use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDigitalOutputReq {
    pub index: u16,
    pub state: bool,
}

impl Default for SetDigitalOutputReq {
    fn default() -> Self {
        SetDigitalOutputReq {
            index: 0,
            state: false,
        }
    }
}

impl ros2_client::Message for SetDigitalOutputReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDigitalOutputRes {
    pub success: bool,
}

impl Default for SetDigitalOutputRes {
    fn default() -> Self {
        SetDigitalOutputRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SetDigitalOutputRes {}


pub struct SetDigitalOutput;
impl ros2_client::Service for SetDigitalOutput {
    type Request = SetDigitalOutputReq;
    type Response = SetDigitalOutputRes;

    fn request_type_name(&self) -> &str { "SetDigitalOutputReq" }
    fn response_type_name(&self) -> &str { "SetDigitalOutputRes" }
}
