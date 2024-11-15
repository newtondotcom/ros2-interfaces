use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteModelReq {
    pub model_name: ::std::string::String,
}

impl Default for DeleteModelReq {
    fn default() -> Self {
        DeleteModelReq {
            model_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteModelReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteModelRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for DeleteModelRes {
    fn default() -> Self {
        DeleteModelRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteModelRes {}


pub struct DeleteModel;
impl ros2_client::Service for DeleteModel {
    type Request = DeleteModelReq;
    type Response = DeleteModelRes;

    fn request_type_name(&self) -> &str { "DeleteModelReq" }
    fn response_type_name(&self) -> &str { "DeleteModelRes" }
}
