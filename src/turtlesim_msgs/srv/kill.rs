use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KillReq {
    pub name: ::std::string::String,
}

impl Default for KillReq {
    fn default() -> Self {
        KillReq {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for KillReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KillRes {

}

impl Default for KillRes {
    fn default() -> Self {
        KillRes {

        }
    }
}

impl ros2_client::Message for KillRes {}


pub struct Kill;
impl ros2_client::Service for Kill {
    type Request = KillReq;
    type Response = KillRes;

    fn request_type_name(&self) -> &str { "KillReq" }
    fn response_type_name(&self) -> &str { "KillRes" }
}
