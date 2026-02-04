use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyPresetWriteRequest {
    pub safety_preset: ::std::string::String,
    pub index: u8,
}

impl Default for SafetyPresetWriteRequest {
    fn default() -> Self {
        SafetyPresetWriteRequest {
            safety_preset: ::std::string::String::new(),
            index: 0,
        }
    }
}

impl ros2_client::Message for SafetyPresetWriteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyPresetWriteResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
}

impl Default for SafetyPresetWriteResponse {
    fn default() -> Self {
        SafetyPresetWriteResponse {
            success: false,
            error_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SafetyPresetWriteResponse {}


pub struct SafetyPresetWrite;
impl ros2_client::Service for SafetyPresetWrite {
    type Request = SafetyPresetWriteRequest;
    type Response = SafetyPresetWriteResponse;

    fn request_type_name(&self) -> &str { "SafetyPresetWriteRequest" }
    fn response_type_name(&self) -> &str { "SafetyPresetWriteResponse" }
}
