use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HardwareMonitorCommandSendRequest {
    pub opcode: u32,
    pub param1: u32,
    pub param2: u32,
    pub param3: u32,
    pub param4: u32,
    pub data: Vec<u8>,
}

impl Default for HardwareMonitorCommandSendRequest {
    fn default() -> Self {
        HardwareMonitorCommandSendRequest {
            opcode: 0,
            param1: 0,
            param2: 0,
            param3: 0,
            param4: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for HardwareMonitorCommandSendRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HardwareMonitorCommandSendResponse {
    pub success: bool,
    pub result: Vec<u8>,
    pub error_message: ::std::string::String,
}

impl Default for HardwareMonitorCommandSendResponse {
    fn default() -> Self {
        HardwareMonitorCommandSendResponse {
            success: false,
            result: Vec::new(),
            error_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for HardwareMonitorCommandSendResponse {}


pub struct HardwareMonitorCommandSend;
impl ros2_client::Service for HardwareMonitorCommandSend {
    type Request = HardwareMonitorCommandSendRequest;
    type Response = HardwareMonitorCommandSendResponse;

    fn request_type_name(&self) -> &str { "HardwareMonitorCommandSendRequest" }
    fn response_type_name(&self) -> &str { "HardwareMonitorCommandSendResponse" }
}
