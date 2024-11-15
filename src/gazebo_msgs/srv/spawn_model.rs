use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnModelReq {
    pub model_name: ::std::string::String,
    pub model_xml: ::std::string::String,
    pub robot_namespace: ::std::string::String,
    pub initial_pose: crate::geometry_msgs::msg::Pose,
    pub reference_frame: ::std::string::String,
}

impl Default for SpawnModelReq {
    fn default() -> Self {
        SpawnModelReq {
            model_name: ::std::string::String::new(),
            model_xml: ::std::string::String::new(),
            robot_namespace: ::std::string::String::new(),
            initial_pose: crate::geometry_msgs::msg::Pose::default(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpawnModelReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnModelRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SpawnModelRes {
    fn default() -> Self {
        SpawnModelRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpawnModelRes {}


pub struct SpawnModel;
impl ros2_client::Service for SpawnModel {
    type Request = SpawnModelReq;
    type Response = SpawnModelRes;

    fn request_type_name(&self) -> &str { "SpawnModelReq" }
    fn response_type_name(&self) -> &str { "SpawnModelRes" }
}
