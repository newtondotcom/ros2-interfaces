use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLinkPropertiesReq {
    pub link_name: ::std::string::String,
    pub com: crate::geometry_msgs::msg::Pose,
    pub gravity_mode: bool,
    pub mass: f64,
    pub ixx: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyy: f64,
    pub iyz: f64,
    pub izz: f64,
}

impl Default for SetLinkPropertiesReq {
    fn default() -> Self {
        SetLinkPropertiesReq {
            link_name: ::std::string::String::new(),
            com: crate::geometry_msgs::msg::Pose::default(),
            gravity_mode: false,
            mass: 0.0,
            ixx: 0.0,
            ixy: 0.0,
            ixz: 0.0,
            iyy: 0.0,
            iyz: 0.0,
            izz: 0.0,
        }
    }
}

impl ros2_client::Message for SetLinkPropertiesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLinkPropertiesRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetLinkPropertiesRes {
    fn default() -> Self {
        SetLinkPropertiesRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetLinkPropertiesRes {}


pub struct SetLinkProperties;
impl ros2_client::Service for SetLinkProperties {
    type Request = SetLinkPropertiesReq;
    type Response = SetLinkPropertiesRes;

    fn request_type_name(&self) -> &str { "SetLinkPropertiesReq" }
    fn response_type_name(&self) -> &str { "SetLinkPropertiesRes" }
}
