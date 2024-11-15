use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiftClearanceReq {
    pub robot_name: ::std::string::String,
    pub lift_name: ::std::string::String,
}

impl Default for LiftClearanceReq {
    fn default() -> Self {
        LiftClearanceReq {
            robot_name: ::std::string::String::new(),
            lift_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LiftClearanceReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiftClearanceRes {
    pub decision: u32,
}

impl LiftClearanceRes {
    pub const DECISION_CLEAR: u32 = 1;
    pub const DECISION_CROWDED: u32 = 2;
}

impl Default for LiftClearanceRes {
    fn default() -> Self {
        LiftClearanceRes {
            decision: 0,
        }
    }
}

impl ros2_client::Message for LiftClearanceRes {}


pub struct LiftClearance;
impl ros2_client::Service for LiftClearance {
    type Request = LiftClearanceReq;
    type Response = LiftClearanceRes;

    fn request_type_name(&self) -> &str { "LiftClearanceReq" }
    fn response_type_name(&self) -> &str { "LiftClearanceRes" }
}
