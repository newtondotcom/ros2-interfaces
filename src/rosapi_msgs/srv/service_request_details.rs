use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceRequestDetailsReq {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for ServiceRequestDetailsReq {
    fn default() -> Self {
        ServiceRequestDetailsReq {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServiceRequestDetailsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceRequestDetailsRes {
    pub typedefs: Vec<crate::rosapi_msgs::msg::TypeDef>,
}

impl Default for ServiceRequestDetailsRes {
    fn default() -> Self {
        ServiceRequestDetailsRes {
            typedefs: Vec::new(),
        }
    }
}

impl ros2_client::Message for ServiceRequestDetailsRes {}


pub struct ServiceRequestDetails;
impl ros2_client::Service for ServiceRequestDetails {
    type Request = ServiceRequestDetailsReq;
    type Response = ServiceRequestDetailsRes;

    fn request_type_name(&self) -> &str { "ServiceRequestDetailsReq" }
    fn response_type_name(&self) -> &str { "ServiceRequestDetailsRes" }
}
