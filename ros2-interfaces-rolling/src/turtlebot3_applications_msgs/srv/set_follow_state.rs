use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetFollowStateRequest {
    pub state: u8,
}

impl SetFollowStateRequest {
    pub const STOPPED: u8 = 0;
    pub const FOLLOW: u8 = 1;
    pub const OK: u8 = 0;
    pub const ERROR: u8 = 1;
}

impl Default for SetFollowStateRequest {
    fn default() -> Self {
        SetFollowStateRequest {
            state: 0,
        }
    }
}

impl ros2_client::Message for SetFollowStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetFollowStateResponse {
    pub result: u8,
}

impl Default for SetFollowStateResponse {
    fn default() -> Self {
        SetFollowStateResponse {
            result: 0,
        }
    }
}

impl ros2_client::Message for SetFollowStateResponse {}


pub struct SetFollowState;
impl ros2_client::Service for SetFollowState {
    type Request = SetFollowStateRequest;
    type Response = SetFollowStateResponse;

    fn request_type_name(&self) -> &str { "SetFollowStateRequest" }
    fn response_type_name(&self) -> &str { "SetFollowStateResponse" }
}
