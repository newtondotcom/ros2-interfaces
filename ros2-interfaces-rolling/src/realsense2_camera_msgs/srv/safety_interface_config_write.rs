use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyInterfaceConfigWriteRequest {
    pub safety_interface_config: ::std::string::String,
}

impl Default for SafetyInterfaceConfigWriteRequest {
    fn default() -> Self {
        SafetyInterfaceConfigWriteRequest {
            safety_interface_config: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SafetyInterfaceConfigWriteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyInterfaceConfigWriteResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
}

impl Default for SafetyInterfaceConfigWriteResponse {
    fn default() -> Self {
        SafetyInterfaceConfigWriteResponse {
            success: false,
            error_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SafetyInterfaceConfigWriteResponse {}


pub struct SafetyInterfaceConfigWrite;
impl ros2_client::Service for SafetyInterfaceConfigWrite {
    type Request = SafetyInterfaceConfigWriteRequest;
    type Response = SafetyInterfaceConfigWriteResponse;

    fn request_type_name(&self) -> &str { "SafetyInterfaceConfigWriteRequest" }
    fn response_type_name(&self) -> &str { "SafetyInterfaceConfigWriteResponse" }
}
