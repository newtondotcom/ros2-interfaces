use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesReq {

}

impl Default for ServicesReq {
    fn default() -> Self {
        ServicesReq {

        }
    }
}

impl ros2_client::Message for ServicesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesRes {
    pub services: Vec<::std::string::String>,
}

impl Default for ServicesRes {
    fn default() -> Self {
        ServicesRes {
            services: Vec::new(),
        }
    }
}

impl ros2_client::Message for ServicesRes {}


pub struct Services;
impl ros2_client::Service for Services {
    type Request = ServicesReq;
    type Response = ServicesRes;

    fn request_type_name(&self) -> &str { "ServicesReq" }
    fn response_type_name(&self) -> &str { "ServicesRes" }
}
