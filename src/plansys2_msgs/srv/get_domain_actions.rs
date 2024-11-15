use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainActionsReq {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetDomainActionsReq {
    fn default() -> Self {
        GetDomainActionsReq {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetDomainActionsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainActionsRes {
    pub success: bool,
    pub actions: Vec<::std::string::String>,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainActionsRes {
    fn default() -> Self {
        GetDomainActionsRes {
            success: false,
            actions: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainActionsRes {}


pub struct GetDomainActions;
impl ros2_client::Service for GetDomainActions {
    type Request = GetDomainActionsReq;
    type Response = GetDomainActionsRes;

    fn request_type_name(&self) -> &str { "GetDomainActionsReq" }
    fn response_type_name(&self) -> &str { "GetDomainActionsRes" }
}
