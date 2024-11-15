use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStateReq {
    pub time_stamp: crate::builtin_interfaces::msg::Time,
    pub frame_id: ::std::string::String,
}

impl Default for GetStateReq {
    fn default() -> Self {
        GetStateReq {
            time_stamp: crate::builtin_interfaces::msg::Time::default(),
            frame_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetStateReq {}


use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStateRes {
    pub state: [f64; 15],
    #[serde_as(as = "[_; 225]")]
    pub covariance: [f64; 225],
}

impl Default for GetStateRes {
    fn default() -> Self {
        GetStateRes {
            state: [0.0; 15],
            covariance: [0.0; 225],
        }
    }
}

impl ros2_client::Message for GetStateRes {}


pub struct GetState;
impl ros2_client::Service for GetState {
    type Request = GetStateReq;
    type Response = GetStateRes;

    fn request_type_name(&self) -> &str { "GetStateReq" }
    fn response_type_name(&self) -> &str { "GetStateRes" }
}
