use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnEntityReq {
    pub name: ::std::string::String,
    pub xml: ::std::string::String,
    pub robot_namespace: ::std::string::String,
    pub initial_pose: crate::geometry_msgs::msg::Pose,
    pub reference_frame: ::std::string::String,
}

impl Default for SpawnEntityReq {
    fn default() -> Self {
        SpawnEntityReq {
            name: ::std::string::String::new(),
            xml: ::std::string::String::new(),
            robot_namespace: ::std::string::String::new(),
            initial_pose: crate::geometry_msgs::msg::Pose::default(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpawnEntityReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnEntityRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SpawnEntityRes {
    fn default() -> Self {
        SpawnEntityRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpawnEntityRes {}


pub struct SpawnEntity;
impl ros2_client::Service for SpawnEntity {
    type Request = SpawnEntityReq;
    type Response = SpawnEntityRes;

    fn request_type_name(&self) -> &str { "SpawnEntityReq" }
    fn response_type_name(&self) -> &str { "SpawnEntityRes" }
}
