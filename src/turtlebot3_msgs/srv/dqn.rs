use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DqnReq {
    pub action: u8,
    pub init: bool,
}

impl Default for DqnReq {
    fn default() -> Self {
        DqnReq {
            action: 0,
            init: false,
        }
    }
}

impl ros2_client::Message for DqnReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DqnRes {
    pub state: Vec<f32>,
    pub reward: f32,
    pub done: bool,
}

impl Default for DqnRes {
    fn default() -> Self {
        DqnRes {
            state: Vec::new(),
            reward: 0.0,
            done: false,
        }
    }
}

impl ros2_client::Message for DqnRes {}


pub struct Dqn;
impl ros2_client::Service for Dqn {
    type Request = DqnReq;
    type Response = DqnRes;

    fn request_type_name(&self) -> &str { "DqnReq" }
    fn response_type_name(&self) -> &str { "DqnRes" }
}
