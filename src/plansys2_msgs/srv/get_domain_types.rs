use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainTypesReq {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetDomainTypesReq {
    fn default() -> Self {
        GetDomainTypesReq {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetDomainTypesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainTypesRes {
    pub success: bool,
    pub types: Vec<::std::string::String>,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainTypesRes {
    fn default() -> Self {
        GetDomainTypesRes {
            success: false,
            types: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainTypesRes {}


pub struct GetDomainTypes;
impl ros2_client::Service for GetDomainTypes {
    type Request = GetDomainTypesReq;
    type Response = GetDomainTypesRes;

    fn request_type_name(&self) -> &str { "GetDomainTypesReq" }
    fn response_type_name(&self) -> &str { "GetDomainTypesRes" }
}
