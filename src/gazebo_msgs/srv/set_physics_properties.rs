use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPhysicsPropertiesReq {
    pub time_step: f64,
    pub max_update_rate: f64,
    pub gravity: crate::geometry_msgs::msg::Vector3,
    pub ode_config: crate::gazebo_msgs::msg::ODEPhysics,
}

impl Default for SetPhysicsPropertiesReq {
    fn default() -> Self {
        SetPhysicsPropertiesReq {
            time_step: 0.0,
            max_update_rate: 0.0,
            gravity: crate::geometry_msgs::msg::Vector3::default(),
            ode_config: crate::gazebo_msgs::msg::ODEPhysics::default(),
        }
    }
}

impl ros2_client::Message for SetPhysicsPropertiesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPhysicsPropertiesRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetPhysicsPropertiesRes {
    fn default() -> Self {
        SetPhysicsPropertiesRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetPhysicsPropertiesRes {}


pub struct SetPhysicsProperties;
impl ros2_client::Service for SetPhysicsProperties {
    type Request = SetPhysicsPropertiesReq;
    type Response = SetPhysicsPropertiesRes;

    fn request_type_name(&self) -> &str { "SetPhysicsPropertiesReq" }
    fn response_type_name(&self) -> &str { "SetPhysicsPropertiesRes" }
}
