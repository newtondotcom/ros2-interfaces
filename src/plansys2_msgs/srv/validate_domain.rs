use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidateDomainReq {
    pub domain: ::std::string::String,
}

impl Default for ValidateDomainReq {
    fn default() -> Self {
        ValidateDomainReq {
            domain: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ValidateDomainReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidateDomainRes {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for ValidateDomainRes {
    fn default() -> Self {
        ValidateDomainRes {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ValidateDomainRes {}


pub struct ValidateDomain;
impl ros2_client::Service for ValidateDomain {
    type Request = ValidateDomainReq;
    type Response = ValidateDomainRes;

    fn request_type_name(&self) -> &str { "ValidateDomainReq" }
    fn response_type_name(&self) -> &str { "ValidateDomainRes" }
}
