use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemInstanceDetailsReq {
    pub instance: ::std::string::String,
}

impl Default for GetProblemInstanceDetailsReq {
    fn default() -> Self {
        GetProblemInstanceDetailsReq {
            instance: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetProblemInstanceDetailsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemInstanceDetailsRes {
    pub success: bool,
    pub instance: crate::plansys2_msgs::msg::Param,
    pub error_info: ::std::string::String,
}

impl Default for GetProblemInstanceDetailsRes {
    fn default() -> Self {
        GetProblemInstanceDetailsRes {
            success: false,
            instance: crate::plansys2_msgs::msg::Param::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetProblemInstanceDetailsRes {}


pub struct GetProblemInstanceDetails;
impl ros2_client::Service for GetProblemInstanceDetails {
    type Request = GetProblemInstanceDetailsReq;
    type Response = GetProblemInstanceDetailsRes;

    fn request_type_name(&self) -> &str { "GetProblemInstanceDetailsReq" }
    fn response_type_name(&self) -> &str { "GetProblemInstanceDetailsRes" }
}
