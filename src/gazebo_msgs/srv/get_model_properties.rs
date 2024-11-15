use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModelPropertiesReq {
    pub model_name: ::std::string::String,
}

impl Default for GetModelPropertiesReq {
    fn default() -> Self {
        GetModelPropertiesReq {
            model_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetModelPropertiesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModelPropertiesRes {
    pub parent_model_name: ::std::string::String,
    pub canonical_body_name: ::std::string::String,
    pub body_names: Vec<::std::string::String>,
    pub geom_names: Vec<::std::string::String>,
    pub joint_names: Vec<::std::string::String>,
    pub child_model_names: Vec<::std::string::String>,
    pub is_static: bool,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetModelPropertiesRes {
    fn default() -> Self {
        GetModelPropertiesRes {
            parent_model_name: ::std::string::String::new(),
            canonical_body_name: ::std::string::String::new(),
            body_names: Vec::new(),
            geom_names: Vec::new(),
            joint_names: Vec::new(),
            child_model_names: Vec::new(),
            is_static: false,
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetModelPropertiesRes {}


pub struct GetModelProperties;
impl ros2_client::Service for GetModelProperties {
    type Request = GetModelPropertiesReq;
    type Response = GetModelPropertiesRes;

    fn request_type_name(&self) -> &str { "GetModelPropertiesReq" }
    fn response_type_name(&self) -> &str { "GetModelPropertiesRes" }
}
