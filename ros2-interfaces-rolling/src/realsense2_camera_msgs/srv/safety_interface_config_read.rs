use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyInterfaceConfigReadRequest {
    pub calib_location: u8, // default: 2
}

impl Default for SafetyInterfaceConfigReadRequest {
    fn default() -> Self {
        SafetyInterfaceConfigReadRequest {
            calib_location: 2,
        }
    }
}

impl ros2_client::Message for SafetyInterfaceConfigReadRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyInterfaceConfigReadResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
    pub safety_interface_config: ::std::string::String,
}

impl Default for SafetyInterfaceConfigReadResponse {
    fn default() -> Self {
        SafetyInterfaceConfigReadResponse {
            success: false,
            error_message: ::std::string::String::new(),
            safety_interface_config: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SafetyInterfaceConfigReadResponse {}


pub struct SafetyInterfaceConfigRead;
impl ros2_client::Service for SafetyInterfaceConfigRead {
    type Request = SafetyInterfaceConfigReadRequest;
    type Response = SafetyInterfaceConfigReadResponse;

    fn request_type_name(&self) -> &str { "SafetyInterfaceConfigReadRequest" }
    fn response_type_name(&self) -> &str { "SafetyInterfaceConfigReadResponse" }
}
