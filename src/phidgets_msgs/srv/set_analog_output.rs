use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAnalogOutputReq {
    pub index: u16,
    pub voltage: f64,
}

impl Default for SetAnalogOutputReq {
    fn default() -> Self {
        SetAnalogOutputReq {
            index: 0,
            voltage: 0.0,
        }
    }
}

impl ros2_client::Message for SetAnalogOutputReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAnalogOutputRes {
    pub success: bool,
}

impl Default for SetAnalogOutputRes {
    fn default() -> Self {
        SetAnalogOutputRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SetAnalogOutputRes {}


pub struct SetAnalogOutput;
impl ros2_client::Service for SetAnalogOutput {
    type Request = SetAnalogOutputReq;
    type Response = SetAnalogOutputRes;

    fn request_type_name(&self) -> &str { "SetAnalogOutputReq" }
    fn response_type_name(&self) -> &str { "SetAnalogOutputRes" }
}
