use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceResponseDetailsReq {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for ServiceResponseDetailsReq {
    fn default() -> Self {
        ServiceResponseDetailsReq {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServiceResponseDetailsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceResponseDetailsRes {
    pub typedefs: Vec<crate::rosapi_msgs::msg::TypeDef>,
}

impl Default for ServiceResponseDetailsRes {
    fn default() -> Self {
        ServiceResponseDetailsRes {
            typedefs: Vec::new(),
        }
    }
}

impl ros2_client::Message for ServiceResponseDetailsRes {}


pub struct ServiceResponseDetails;
impl ros2_client::Service for ServiceResponseDetails {
    type Request = ServiceResponseDetailsReq;
    type Response = ServiceResponseDetailsRes;

    fn request_type_name(&self) -> &str { "ServiceResponseDetailsReq" }
    fn response_type_name(&self) -> &str { "ServiceResponseDetailsRes" }
}
