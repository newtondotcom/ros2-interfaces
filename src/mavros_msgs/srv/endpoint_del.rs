use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDelReq {
    pub id: u32,
    pub url: ::std::string::String,
    #[serde(rename = "type")]    pub type_: u8,
}

impl EndpointDelReq {
    pub const TYPE_FCU: u8 = 0;
    pub const TYPE_GCS: u8 = 1;
    pub const TYPE_UAS: u8 = 2;
}

impl Default for EndpointDelReq {
    fn default() -> Self {
        EndpointDelReq {
            id: 0,
            url: ::std::string::String::new(),
            type_: 0,
        }
    }
}

impl ros2_client::Message for EndpointDelReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDelRes {
    pub successful: bool,
}

impl Default for EndpointDelRes {
    fn default() -> Self {
        EndpointDelRes {
            successful: false,
        }
    }
}

impl ros2_client::Message for EndpointDelRes {}


pub struct EndpointDel;
impl ros2_client::Service for EndpointDel {
    type Request = EndpointDelReq;
    type Response = EndpointDelRes;

    fn request_type_name(&self) -> &str { "EndpointDelReq" }
    fn response_type_name(&self) -> &str { "EndpointDelRes" }
}
