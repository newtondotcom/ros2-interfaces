use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorldPropertiesReq {

}

impl Default for GetWorldPropertiesReq {
    fn default() -> Self {
        GetWorldPropertiesReq {

        }
    }
}

impl ros2_client::Message for GetWorldPropertiesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorldPropertiesRes {
    pub sim_time: f64,
    pub model_names: Vec<::std::string::String>,
    pub rendering_enabled: bool,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetWorldPropertiesRes {
    fn default() -> Self {
        GetWorldPropertiesRes {
            sim_time: 0.0,
            model_names: Vec::new(),
            rendering_enabled: false,
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetWorldPropertiesRes {}


pub struct GetWorldProperties;
impl ros2_client::Service for GetWorldProperties {
    type Request = GetWorldPropertiesReq;
    type Response = GetWorldPropertiesRes;

    fn request_type_name(&self) -> &str { "GetWorldPropertiesReq" }
    fn response_type_name(&self) -> &str { "GetWorldPropertiesRes" }
}
