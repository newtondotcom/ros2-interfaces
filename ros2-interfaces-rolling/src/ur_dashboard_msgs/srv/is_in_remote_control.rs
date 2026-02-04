use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsInRemoteControlRequest {

}

impl Default for IsInRemoteControlRequest {
    fn default() -> Self {
        IsInRemoteControlRequest {

        }
    }
}

impl ros2_client::Message for IsInRemoteControlRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsInRemoteControlResponse {
    pub answer: ::std::string::String,
    pub remote_control: bool,
    pub success: bool,
}

impl Default for IsInRemoteControlResponse {
    fn default() -> Self {
        IsInRemoteControlResponse {
            answer: ::std::string::String::new(),
            remote_control: false,
            success: false,
        }
    }
}

impl ros2_client::Message for IsInRemoteControlResponse {}


pub struct IsInRemoteControl;
impl ros2_client::Service for IsInRemoteControl {
    type Request = IsInRemoteControlRequest;
    type Response = IsInRemoteControlResponse;

    fn request_type_name(&self) -> &str { "IsInRemoteControlRequest" }
    fn response_type_name(&self) -> &str { "IsInRemoteControlResponse" }
}
