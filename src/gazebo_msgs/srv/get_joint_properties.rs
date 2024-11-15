use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetJointPropertiesReq {
    pub joint_name: ::std::string::String,
}

impl Default for GetJointPropertiesReq {
    fn default() -> Self {
        GetJointPropertiesReq {
            joint_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetJointPropertiesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetJointPropertiesRes {
    #[serde(rename = "type")]    pub type_: u8,
    pub damping: Vec<f64>,
    pub position: Vec<f64>,
    pub rate: Vec<f64>,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl GetJointPropertiesRes {
    pub const REVOLUTE: u8 = 0;
    pub const CONTINUOUS: u8 = 1;
    pub const PRISMATIC: u8 = 2;
    pub const FIXED: u8 = 3;
    pub const BALL: u8 = 4;
    pub const UNIVERSAL: u8 = 5;
}

impl Default for GetJointPropertiesRes {
    fn default() -> Self {
        GetJointPropertiesRes {
            type_: 0,
            damping: Vec::new(),
            position: Vec::new(),
            rate: Vec::new(),
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetJointPropertiesRes {}


pub struct GetJointProperties;
impl ros2_client::Service for GetJointProperties {
    type Request = GetJointPropertiesReq;
    type Response = GetJointPropertiesRes;

    fn request_type_name(&self) -> &str { "GetJointPropertiesReq" }
    fn response_type_name(&self) -> &str { "GetJointPropertiesRes" }
}
