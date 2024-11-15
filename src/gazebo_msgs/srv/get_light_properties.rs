use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLightPropertiesReq {
    pub light_name: ::std::string::String,
}

impl Default for GetLightPropertiesReq {
    fn default() -> Self {
        GetLightPropertiesReq {
            light_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetLightPropertiesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLightPropertiesRes {
    pub diffuse: crate::std_msgs::msg::ColorRGBA,
    pub attenuation_constant: f64,
    pub attenuation_linear: f64,
    pub attenuation_quadratic: f64,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetLightPropertiesRes {
    fn default() -> Self {
        GetLightPropertiesRes {
            diffuse: crate::std_msgs::msg::ColorRGBA::default(),
            attenuation_constant: 0.0,
            attenuation_linear: 0.0,
            attenuation_quadratic: 0.0,
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetLightPropertiesRes {}


pub struct GetLightProperties;
impl ros2_client::Service for GetLightProperties {
    type Request = GetLightPropertiesReq;
    type Response = GetLightPropertiesRes;

    fn request_type_name(&self) -> &str { "GetLightPropertiesReq" }
    fn response_type_name(&self) -> &str { "GetLightPropertiesRes" }
}
