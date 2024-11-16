use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxSelectReq {
    pub topic: ::std::string::String,
}

impl Default for MuxSelectReq {
    fn default() -> Self {
        MuxSelectReq {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MuxSelectReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxSelectRes {
    pub prev_topic: ::std::string::String,
    pub success: bool,
}

impl Default for MuxSelectRes {
    fn default() -> Self {
        MuxSelectRes {
            prev_topic: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for MuxSelectRes {}


pub struct MuxSelect;
impl ros2_client::Service for MuxSelect {
    type Request = MuxSelectReq;
    type Response = MuxSelectRes;

    fn request_type_name(&self) -> &str { "MuxSelectReq" }
    fn response_type_name(&self) -> &str { "MuxSelectRes" }
}
