use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeJointsRequest {
    pub group: ::std::string::String,
    pub joints_to_remove: Vec<::std::string::String>,
    pub joints_to_add: Vec<::std::string::String>,
}

impl Default for ChangeJointsRequest {
    fn default() -> Self {
        ChangeJointsRequest {
            group: ::std::string::String::new(),
            joints_to_remove: Vec::new(),
            joints_to_add: Vec::new(),
        }
    }
}

impl ros2_client::Message for ChangeJointsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeJointsResponse {
    pub ok: bool,
    pub message: ::std::string::String,
    pub current_group: ::std::string::String,
    pub used_joints: Vec<::std::string::String>,
}

impl Default for ChangeJointsResponse {
    fn default() -> Self {
        ChangeJointsResponse {
            ok: false,
            message: ::std::string::String::new(),
            current_group: ::std::string::String::new(),
            used_joints: Vec::new(),
        }
    }
}

impl ros2_client::Message for ChangeJointsResponse {}


pub struct ChangeJoints;
impl ros2_client::Service for ChangeJoints {
    type Request = ChangeJointsRequest;
    type Response = ChangeJointsResponse;

    fn request_type_name(&self) -> &str { "ChangeJointsRequest" }
    fn response_type_name(&self) -> &str { "ChangeJointsResponse" }
}
