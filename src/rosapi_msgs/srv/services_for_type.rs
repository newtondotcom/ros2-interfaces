use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesForTypeReq {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for ServicesForTypeReq {
    fn default() -> Self {
        ServicesForTypeReq {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServicesForTypeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesForTypeRes {
    pub services: Vec<::std::string::String>,
}

impl Default for ServicesForTypeRes {
    fn default() -> Self {
        ServicesForTypeRes {
            services: Vec::new(),
        }
    }
}

impl ros2_client::Message for ServicesForTypeRes {}


pub struct ServicesForType;
impl ros2_client::Service for ServicesForType {
    type Request = ServicesForTypeReq;
    type Response = ServicesForTypeRes;

    fn request_type_name(&self) -> &str { "ServicesForTypeReq" }
    fn response_type_name(&self) -> &str { "ServicesForTypeRes" }
}
