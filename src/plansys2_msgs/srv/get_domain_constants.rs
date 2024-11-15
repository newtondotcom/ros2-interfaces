use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainConstantsReq {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for GetDomainConstantsReq {
    fn default() -> Self {
        GetDomainConstantsReq {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainConstantsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainConstantsRes {
    pub success: bool,
    pub constants: Vec<::std::string::String>,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainConstantsRes {
    fn default() -> Self {
        GetDomainConstantsRes {
            success: false,
            constants: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainConstantsRes {}


pub struct GetDomainConstants;
impl ros2_client::Service for GetDomainConstants {
    type Request = GetDomainConstantsReq;
    type Response = GetDomainConstantsRes;

    fn request_type_name(&self) -> &str { "GetDomainConstantsReq" }
    fn response_type_name(&self) -> &str { "GetDomainConstantsRes" }
}
