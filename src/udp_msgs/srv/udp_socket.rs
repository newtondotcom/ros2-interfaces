use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UdpSocketReq {
    pub local_address: ::std::string::String,
    pub local_port: u16,
    pub remote_address: ::std::string::String,
    pub remote_port: u16,
    pub is_broadcast: bool,
}

impl Default for UdpSocketReq {
    fn default() -> Self {
        UdpSocketReq {
            local_address: ::std::string::String::new(),
            local_port: 0,
            remote_address: ::std::string::String::new(),
            remote_port: 0,
            is_broadcast: false,
        }
    }
}

impl ros2_client::Message for UdpSocketReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UdpSocketRes {
    pub socket_created: bool,
}

impl Default for UdpSocketRes {
    fn default() -> Self {
        UdpSocketRes {
            socket_created: false,
        }
    }
}

impl ros2_client::Message for UdpSocketRes {}


pub struct UdpSocket;
impl ros2_client::Service for UdpSocket {
    type Request = UdpSocketReq;
    type Response = UdpSocketRes;

    fn request_type_name(&self) -> &str { "UdpSocketReq" }
    fn response_type_name(&self) -> &str { "UdpSocketRes" }
}
