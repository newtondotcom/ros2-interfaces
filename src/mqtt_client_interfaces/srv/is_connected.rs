use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsConnectedReq {

}

impl Default for IsConnectedReq {
    fn default() -> Self {
        IsConnectedReq {

        }
    }
}

impl ros2_client::Message for IsConnectedReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsConnectedRes {
    pub connected: bool,
}

impl Default for IsConnectedRes {
    fn default() -> Self {
        IsConnectedRes {
            connected: false,
        }
    }
}

impl ros2_client::Message for IsConnectedRes {}


pub struct IsConnected;
impl ros2_client::Service for IsConnected {
    type Request = IsConnectedReq;
    type Response = IsConnectedRes;

    fn request_type_name(&self) -> &str { "IsConnectedReq" }
    fn response_type_name(&self) -> &str { "IsConnectedRes" }
}
