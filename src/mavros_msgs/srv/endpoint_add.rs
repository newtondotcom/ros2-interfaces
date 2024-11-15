use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointAddReq {
    pub url: ::std::string::String,
    #[serde(rename = "type")]    pub type_: u8,
}

impl EndpointAddReq {
    pub const TYPE_FCU: u8 = 0;
    pub const TYPE_GCS: u8 = 1;
    pub const TYPE_UAS: u8 = 2;
}

impl Default for EndpointAddReq {
    fn default() -> Self {
        EndpointAddReq {
            url: ::std::string::String::new(),
            type_: 0,
        }
    }
}

impl ros2_client::Message for EndpointAddReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointAddRes {
    pub successful: bool,
    pub reason: ::std::string::String,
    pub id: u32,
}

impl Default for EndpointAddRes {
    fn default() -> Self {
        EndpointAddRes {
            successful: false,
            reason: ::std::string::String::new(),
            id: 0,
        }
    }
}

impl ros2_client::Message for EndpointAddRes {}


pub struct EndpointAdd;
impl ros2_client::Service for EndpointAdd {
    type Request = EndpointAddReq;
    type Response = EndpointAddRes;

    fn request_type_name(&self) -> &str { "EndpointAddReq" }
    fn response_type_name(&self) -> &str { "EndpointAddRes" }
}
