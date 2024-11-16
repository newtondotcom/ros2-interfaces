use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteParamReq {
    pub name: ::std::string::String,
}

impl Default for DeleteParamReq {
    fn default() -> Self {
        DeleteParamReq {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteParamReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteParamRes {

}

impl Default for DeleteParamRes {
    fn default() -> Self {
        DeleteParamRes {

        }
    }
}

impl ros2_client::Message for DeleteParamRes {}


pub struct DeleteParam;
impl ros2_client::Service for DeleteParam {
    type Request = DeleteParamReq;
    type Response = DeleteParamRes;

    fn request_type_name(&self) -> &str { "DeleteParamReq" }
    fn response_type_name(&self) -> &str { "DeleteParamRes" }
}
