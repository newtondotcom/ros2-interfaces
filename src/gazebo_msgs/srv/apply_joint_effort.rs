use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyJointEffortReq {
    pub joint_name: ::std::string::String,
    pub effort: f64,
    pub start_time: crate::builtin_interfaces::msg::Time,
    pub duration: crate::builtin_interfaces::msg::Duration,
}

impl Default for ApplyJointEffortReq {
    fn default() -> Self {
        ApplyJointEffortReq {
            joint_name: ::std::string::String::new(),
            effort: 0.0,
            start_time: crate::builtin_interfaces::msg::Time::default(),
            duration: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for ApplyJointEffortReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyJointEffortRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for ApplyJointEffortRes {
    fn default() -> Self {
        ApplyJointEffortRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ApplyJointEffortRes {}


pub struct ApplyJointEffort;
impl ros2_client::Service for ApplyJointEffort {
    type Request = ApplyJointEffortReq;
    type Response = ApplyJointEffortRes;

    fn request_type_name(&self) -> &str { "ApplyJointEffortReq" }
    fn response_type_name(&self) -> &str { "ApplyJointEffortRes" }
}
