use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceProvidersReq {
    pub service: ::std::string::String,
}

impl Default for ServiceProvidersReq {
    fn default() -> Self {
        ServiceProvidersReq {
            service: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServiceProvidersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceProvidersRes {
    pub providers: Vec<::std::string::String>,
}

impl Default for ServiceProvidersRes {
    fn default() -> Self {
        ServiceProvidersRes {
            providers: Vec::new(),
        }
    }
}

impl ros2_client::Message for ServiceProvidersRes {}


pub struct ServiceProviders;
impl ros2_client::Service for ServiceProviders {
    type Request = ServiceProvidersReq;
    type Response = ServiceProvidersRes;

    fn request_type_name(&self) -> &str { "ServiceProvidersReq" }
    fn response_type_name(&self) -> &str { "ServiceProvidersRes" }
}
