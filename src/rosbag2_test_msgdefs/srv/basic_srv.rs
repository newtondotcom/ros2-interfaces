use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicSrvReq {
    pub req: ::std::string::String,
}

impl Default for BasicSrvReq {
    fn default() -> Self {
        BasicSrvReq {
            req: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for BasicSrvReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicSrvRes {
    pub resp: ::std::string::String,
}

impl Default for BasicSrvRes {
    fn default() -> Self {
        BasicSrvRes {
            resp: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for BasicSrvRes {}


pub struct BasicSrv;
impl ros2_client::Service for BasicSrv {
    type Request = BasicSrvReq;
    type Response = BasicSrvRes;

    fn request_type_name(&self) -> &str { "BasicSrvReq" }
    fn response_type_name(&self) -> &str { "BasicSrvRes" }
}
