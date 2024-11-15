use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainDurativeActionDetailsReq {
    pub durative_action: ::std::string::String,
    pub parameters: Vec<::std::string::String>,
}

impl Default for GetDomainDurativeActionDetailsReq {
    fn default() -> Self {
        GetDomainDurativeActionDetailsReq {
            durative_action: ::std::string::String::new(),
            parameters: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetDomainDurativeActionDetailsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainDurativeActionDetailsRes {
    pub success: bool,
    pub durative_action: crate::plansys2_msgs::msg::DurativeAction,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainDurativeActionDetailsRes {
    fn default() -> Self {
        GetDomainDurativeActionDetailsRes {
            success: false,
            durative_action: crate::plansys2_msgs::msg::DurativeAction::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainDurativeActionDetailsRes {}


pub struct GetDomainDurativeActionDetails;
impl ros2_client::Service for GetDomainDurativeActionDetails {
    type Request = GetDomainDurativeActionDetailsReq;
    type Response = GetDomainDurativeActionDetailsRes;

    fn request_type_name(&self) -> &str { "GetDomainDurativeActionDetailsReq" }
    fn response_type_name(&self) -> &str { "GetDomainDurativeActionDetailsRes" }
}
