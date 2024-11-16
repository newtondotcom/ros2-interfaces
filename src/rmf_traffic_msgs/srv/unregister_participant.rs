use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnregisterParticipantReq {
    pub participant_id: u64,
}

impl Default for UnregisterParticipantReq {
    fn default() -> Self {
        UnregisterParticipantReq {
            participant_id: 0,
        }
    }
}

impl ros2_client::Message for UnregisterParticipantReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnregisterParticipantRes {
    pub confirmation: bool,
    pub error: ::std::string::String,
}

impl Default for UnregisterParticipantRes {
    fn default() -> Self {
        UnregisterParticipantRes {
            confirmation: false,
            error: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UnregisterParticipantRes {}


pub struct UnregisterParticipant;
impl ros2_client::Service for UnregisterParticipant {
    type Request = UnregisterParticipantReq;
    type Response = UnregisterParticipantRes;

    fn request_type_name(&self) -> &str { "UnregisterParticipantReq" }
    fn response_type_name(&self) -> &str { "UnregisterParticipantRes" }
}
