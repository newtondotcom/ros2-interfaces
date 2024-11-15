use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetJointPropertiesReq {
    pub joint_name: ::std::string::String,
    pub ode_joint_config: crate::gazebo_msgs::msg::ODEJointProperties,
}

impl Default for SetJointPropertiesReq {
    fn default() -> Self {
        SetJointPropertiesReq {
            joint_name: ::std::string::String::new(),
            ode_joint_config: crate::gazebo_msgs::msg::ODEJointProperties::default(),
        }
    }
}

impl ros2_client::Message for SetJointPropertiesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetJointPropertiesRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetJointPropertiesRes {
    fn default() -> Self {
        SetJointPropertiesRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetJointPropertiesRes {}


pub struct SetJointProperties;
impl ros2_client::Service for SetJointProperties {
    type Request = SetJointPropertiesReq;
    type Response = SetJointPropertiesRes;

    fn request_type_name(&self) -> &str { "SetJointPropertiesReq" }
    fn response_type_name(&self) -> &str { "SetJointPropertiesRes" }
}
