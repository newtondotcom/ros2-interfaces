use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColaMsgSrvReq {
    pub request: ::std::string::String,
}

impl Default for ColaMsgSrvReq {
    fn default() -> Self {
        ColaMsgSrvReq {
            request: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ColaMsgSrvReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColaMsgSrvRes {
    pub response: ::std::string::String,
}

impl Default for ColaMsgSrvRes {
    fn default() -> Self {
        ColaMsgSrvRes {
            response: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ColaMsgSrvRes {}


pub struct ColaMsgSrv;
impl ros2_client::Service for ColaMsgSrv {
    type Request = ColaMsgSrvReq;
    type Response = ColaMsgSrvRes;

    fn request_type_name(&self) -> &str { "ColaMsgSrvReq" }
    fn response_type_name(&self) -> &str { "ColaMsgSrvRes" }
}
