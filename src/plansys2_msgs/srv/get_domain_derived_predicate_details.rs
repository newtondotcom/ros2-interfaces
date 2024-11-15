use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainDerivedPredicateDetailsReq {
    pub predicate: ::std::string::String,
}

impl Default for GetDomainDerivedPredicateDetailsReq {
    fn default() -> Self {
        GetDomainDerivedPredicateDetailsReq {
            predicate: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainDerivedPredicateDetailsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainDerivedPredicateDetailsRes {
    pub predicates: Vec<crate::plansys2_msgs::msg::Derived>,
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainDerivedPredicateDetailsRes {
    fn default() -> Self {
        GetDomainDerivedPredicateDetailsRes {
            predicates: Vec::new(),
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetDomainDerivedPredicateDetailsRes {}


pub struct GetDomainDerivedPredicateDetails;
impl ros2_client::Service for GetDomainDerivedPredicateDetails {
    type Request = GetDomainDerivedPredicateDetailsReq;
    type Response = GetDomainDerivedPredicateDetailsRes;

    fn request_type_name(&self) -> &str { "GetDomainDerivedPredicateDetailsReq" }
    fn response_type_name(&self) -> &str { "GetDomainDerivedPredicateDetailsRes" }
}
