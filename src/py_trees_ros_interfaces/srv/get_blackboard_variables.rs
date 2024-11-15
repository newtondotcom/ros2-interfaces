use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlackboardVariablesReq {

}

impl Default for GetBlackboardVariablesReq {
    fn default() -> Self {
        GetBlackboardVariablesReq {

        }
    }
}

impl ros2_client::Message for GetBlackboardVariablesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlackboardVariablesRes {
    pub variables: Vec<::std::string::String>,
}

impl Default for GetBlackboardVariablesRes {
    fn default() -> Self {
        GetBlackboardVariablesRes {
            variables: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetBlackboardVariablesRes {}


pub struct GetBlackboardVariables;
impl ros2_client::Service for GetBlackboardVariables {
    type Request = GetBlackboardVariablesReq;
    type Response = GetBlackboardVariablesRes;

    fn request_type_name(&self) -> &str { "GetBlackboardVariablesReq" }
    fn response_type_name(&self) -> &str { "GetBlackboardVariablesRes" }
}
