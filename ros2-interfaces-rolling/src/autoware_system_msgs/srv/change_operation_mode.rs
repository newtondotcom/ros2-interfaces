use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeOperationModeRequest {
    pub mode: u16,
}

impl ChangeOperationModeRequest {
    pub const STOP: u16 = 1;
    pub const AUTONOMOUS: u16 = 2;
    pub const LOCAL: u16 = 3;
    pub const REMOTE: u16 = 4;
}

impl Default for ChangeOperationModeRequest {
    fn default() -> Self {
        ChangeOperationModeRequest {
            mode: 0,
        }
    }
}

impl ros2_client::Message for ChangeOperationModeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeOperationModeResponse {
    pub status: crate::autoware_common_msgs::msg::ResponseStatus,
}

impl Default for ChangeOperationModeResponse {
    fn default() -> Self {
        ChangeOperationModeResponse {
            status: crate::autoware_common_msgs::msg::ResponseStatus::default(),
        }
    }
}

impl ros2_client::Message for ChangeOperationModeResponse {}


pub struct ChangeOperationMode;
impl ros2_client::Service for ChangeOperationMode {
    type Request = ChangeOperationModeRequest;
    type Response = ChangeOperationModeResponse;

    fn request_type_name(&self) -> &str { "ChangeOperationModeRequest" }
    fn response_type_name(&self) -> &str { "ChangeOperationModeResponse" }
}
