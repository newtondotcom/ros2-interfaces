use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UdpSendReq {
    pub local_address: ::std::string::String,
    pub local_port: u16,
    pub remote_address: ::std::string::String,
    pub remote_port: u16,
    pub data: Vec<u8>,
}

impl Default for UdpSendReq {
    fn default() -> Self {
        UdpSendReq {
            local_address: ::std::string::String::new(),
            local_port: 0,
            remote_address: ::std::string::String::new(),
            remote_port: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for UdpSendReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UdpSendRes {
    pub sent: bool,
}

impl Default for UdpSendRes {
    fn default() -> Self {
        UdpSendRes {
            sent: false,
        }
    }
}

impl ros2_client::Message for UdpSendRes {}


pub struct UdpSend;
impl ros2_client::Service for UdpSend {
    type Request = UdpSendReq;
    type Response = UdpSendRes;

    fn request_type_name(&self) -> &str { "UdpSendReq" }
    fn response_type_name(&self) -> &str { "UdpSendRes" }
}
