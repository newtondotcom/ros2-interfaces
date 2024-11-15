use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainActionDetailsReq {
    pub action: ::std::string::String,
    pub parameters: Vec<::std::string::String>,
}

impl Default for GetDomainActionDetailsReq {
    fn default() -> Self {
        GetDomainActionDetailsReq {
            action: ::std::string::String::new(),
            parameters: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetDomainActionDetailsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainActionDetailsRes {
    pub action: crate::plansys2_msgs::msg::Action,
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainActionDetailsRes {
    fn default() -> Self {
        GetDomainActionDetailsRes {
            action: crate::plansys2_msgs::msg::Action::default(),
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainActionDetailsRes {}


pub struct GetDomainActionDetails;
impl ros2_client::Service for GetDomainActionDetails {
    type Request = GetDomainActionDetailsReq;
    type Response = GetDomainActionDetailsRes;

    fn request_type_name(&self) -> &str { "GetDomainActionDetailsReq" }
    fn response_type_name(&self) -> &str { "GetDomainActionDetailsRes" }
}
