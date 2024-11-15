use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainReq {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetDomainReq {
    fn default() -> Self {
        GetDomainReq {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetDomainReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainRes {
    pub success: bool,
    pub domain: ::std::string::String,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainRes {
    fn default() -> Self {
        GetDomainRes {
            success: false,
            domain: ::std::string::String::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainRes {}


pub struct GetDomain;
impl ros2_client::Service for GetDomain {
    type Request = GetDomainReq;
    type Response = GetDomainRes;

    fn request_type_name(&self) -> &str { "GetDomainReq" }
    fn response_type_name(&self) -> &str { "GetDomainRes" }
}
