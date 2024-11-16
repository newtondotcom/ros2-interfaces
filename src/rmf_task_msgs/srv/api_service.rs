use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiServiceReq {
    pub json_msg: ::std::string::String,
}

impl Default for ApiServiceReq {
    fn default() -> Self {
        ApiServiceReq {
            json_msg: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ApiServiceReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiServiceRes {
    pub json_msg: ::std::string::String,
}

impl Default for ApiServiceRes {
    fn default() -> Self {
        ApiServiceRes {
            json_msg: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ApiServiceRes {}


pub struct ApiService;
impl ros2_client::Service for ApiService {
    type Request = ApiServiceReq;
    type Response = ApiServiceRes;

    fn request_type_name(&self) -> &str { "ApiServiceReq" }
    fn response_type_name(&self) -> &str { "ApiServiceRes" }
}
