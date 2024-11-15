use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLightPropertiesReq {
    pub light_name: ::std::string::String,
    pub diffuse: crate::std_msgs::msg::ColorRGBA,
    pub attenuation_constant: f64,
    pub attenuation_linear: f64,
    pub attenuation_quadratic: f64,
}

impl Default for SetLightPropertiesReq {
    fn default() -> Self {
        SetLightPropertiesReq {
            light_name: ::std::string::String::new(),
            diffuse: crate::std_msgs::msg::ColorRGBA::default(),
            attenuation_constant: 0.0,
            attenuation_linear: 0.0,
            attenuation_quadratic: 0.0,
        }
    }
}

impl ros2_client::Message for SetLightPropertiesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLightPropertiesRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetLightPropertiesRes {
    fn default() -> Self {
        SetLightPropertiesRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetLightPropertiesRes {}


pub struct SetLightProperties;
impl ros2_client::Service for SetLightProperties {
    type Request = SetLightPropertiesReq;
    type Response = SetLightPropertiesRes;

    fn request_type_name(&self) -> &str { "SetLightPropertiesReq" }
    fn response_type_name(&self) -> &str { "SetLightPropertiesRes" }
}
