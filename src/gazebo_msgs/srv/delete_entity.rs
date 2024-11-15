use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEntityReq {
    pub name: ::std::string::String,
}

impl Default for DeleteEntityReq {
    fn default() -> Self {
        DeleteEntityReq {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteEntityReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEntityRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for DeleteEntityRes {
    fn default() -> Self {
        DeleteEntityRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteEntityRes {}


pub struct DeleteEntity;
impl ros2_client::Service for DeleteEntity {
    type Request = DeleteEntityReq;
    type Response = DeleteEntityRes;

    fn request_type_name(&self) -> &str { "DeleteEntityReq" }
    fn response_type_name(&self) -> &str { "DeleteEntityRes" }
}
