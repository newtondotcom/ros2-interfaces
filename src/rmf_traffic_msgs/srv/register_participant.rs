use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterParticipantReq {
    pub description: crate::rmf_traffic_msgs::msg::ParticipantDescription,
}

impl Default for RegisterParticipantReq {
    fn default() -> Self {
        RegisterParticipantReq {
            description: crate::rmf_traffic_msgs::msg::ParticipantDescription::default(),
        }
    }
}

impl ros2_client::Message for RegisterParticipantReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterParticipantRes {
    pub participant_id: u64,
    pub last_itinerary_version: u64,
    pub last_plan_id: u64,
    pub next_storage_base: u64,
    pub error: ::std::string::String,
}

impl Default for RegisterParticipantRes {
    fn default() -> Self {
        RegisterParticipantRes {
            participant_id: 0,
            last_itinerary_version: 0,
            last_plan_id: 0,
            next_storage_base: 0,
            error: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RegisterParticipantRes {}


pub struct RegisterParticipant;
impl ros2_client::Service for RegisterParticipant {
    type Request = RegisterParticipantReq;
    type Response = RegisterParticipantRes;

    fn request_type_name(&self) -> &str { "RegisterParticipantReq" }
    fn response_type_name(&self) -> &str { "RegisterParticipantRes" }
}
