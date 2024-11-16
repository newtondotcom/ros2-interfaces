use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceTypeReq {
    pub service: ::std::string::String,
}

impl Default for ServiceTypeReq {
    fn default() -> Self {
        ServiceTypeReq {
            service: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServiceTypeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceTypeRes {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for ServiceTypeRes {
    fn default() -> Self {
        ServiceTypeRes {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServiceTypeRes {}


pub struct ServiceType;
impl ros2_client::Service for ServiceType {
    type Request = ServiceTypeReq;
    type Response = ServiceTypeRes;

    fn request_type_name(&self) -> &str { "ServiceTypeReq" }
    fn response_type_name(&self) -> &str { "ServiceTypeRes" }
}
