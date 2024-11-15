use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetModelConfigurationReq {
    pub model_name: ::std::string::String,
    pub urdf_param_name: ::std::string::String,
    pub joint_names: Vec<::std::string::String>,
    pub joint_positions: Vec<f64>,
}

impl Default for SetModelConfigurationReq {
    fn default() -> Self {
        SetModelConfigurationReq {
            model_name: ::std::string::String::new(),
            urdf_param_name: ::std::string::String::new(),
            joint_names: Vec::new(),
            joint_positions: Vec::new(),
        }
    }
}

impl ros2_client::Message for SetModelConfigurationReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetModelConfigurationRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetModelConfigurationRes {
    fn default() -> Self {
        SetModelConfigurationRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetModelConfigurationRes {}


pub struct SetModelConfiguration;
impl ros2_client::Service for SetModelConfiguration {
    type Request = SetModelConfigurationReq;
    type Response = SetModelConfigurationRes;

    fn request_type_name(&self) -> &str { "SetModelConfigurationReq" }
    fn response_type_name(&self) -> &str { "SetModelConfigurationRes" }
}
