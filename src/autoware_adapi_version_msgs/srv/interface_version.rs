use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceVersionReq {

}

impl Default for InterfaceVersionReq {
    fn default() -> Self {
        InterfaceVersionReq {

        }
    }
}

impl ros2_client::Message for InterfaceVersionReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceVersionRes {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Default for InterfaceVersionRes {
    fn default() -> Self {
        InterfaceVersionRes {
            major: 0,
            minor: 0,
            patch: 0,
        }
    }
}

impl ros2_client::Message for InterfaceVersionRes {}


pub struct InterfaceVersion;
impl ros2_client::Service for InterfaceVersion {
    type Request = InterfaceVersionReq;
    type Response = InterfaceVersionRes;

    fn request_type_name(&self) -> &str { "InterfaceVersionReq" }
    fn response_type_name(&self) -> &str { "InterfaceVersionRes" }
}
