use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyPresetReadRequest {
    pub index: u8,
}

impl Default for SafetyPresetReadRequest {
    fn default() -> Self {
        SafetyPresetReadRequest {
            index: 0,
        }
    }
}

impl ros2_client::Message for SafetyPresetReadRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyPresetReadResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
    pub safety_preset: ::std::string::String,
}

impl Default for SafetyPresetReadResponse {
    fn default() -> Self {
        SafetyPresetReadResponse {
            success: false,
            error_message: ::std::string::String::new(),
            safety_preset: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SafetyPresetReadResponse {}


pub struct SafetyPresetRead;
impl ros2_client::Service for SafetyPresetRead {
    type Request = SafetyPresetReadRequest;
    type Response = SafetyPresetReadResponse;

    fn request_type_name(&self) -> &str { "SafetyPresetReadRequest" }
    fn response_type_name(&self) -> &str { "SafetyPresetReadResponse" }
}
