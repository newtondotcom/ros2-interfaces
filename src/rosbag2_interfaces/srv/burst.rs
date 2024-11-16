use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurstReq {
    pub num_messages: u64,
}

impl Default for BurstReq {
    fn default() -> Self {
        BurstReq {
            num_messages: 0,
        }
    }
}

impl ros2_client::Message for BurstReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurstRes {
    pub actually_burst: u64,
}

impl Default for BurstRes {
    fn default() -> Self {
        BurstRes {
            actually_burst: 0,
        }
    }
}

impl ros2_client::Message for BurstRes {}


pub struct Burst;
impl ros2_client::Service for Burst {
    type Request = BurstReq;
    type Response = BurstRes;

    fn request_type_name(&self) -> &str { "BurstReq" }
    fn response_type_name(&self) -> &str { "BurstRes" }
}
