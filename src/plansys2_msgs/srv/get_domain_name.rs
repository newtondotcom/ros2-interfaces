use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainNameReq {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetDomainNameReq {
    fn default() -> Self {
        GetDomainNameReq {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetDomainNameReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainNameRes {
    pub success: bool,
    pub name: ::std::string::String,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainNameRes {
    fn default() -> Self {
        GetDomainNameRes {
            success: false,
            name: ::std::string::String::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainNameRes {}


pub struct GetDomainName;
impl ros2_client::Service for GetDomainName {
    type Request = GetDomainNameReq;
    type Response = GetDomainNameRes;

    fn request_type_name(&self) -> &str { "GetDomainNameReq" }
    fn response_type_name(&self) -> &str { "GetDomainNameRes" }
}
