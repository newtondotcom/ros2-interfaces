use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmptyReq {

}

impl Default for EmptyReq {
    fn default() -> Self {
        EmptyReq {

        }
    }
}

impl ros2_client::Message for EmptyReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmptyRes {

}

impl Default for EmptyRes {
    fn default() -> Self {
        EmptyRes {

        }
    }
}

impl ros2_client::Message for EmptyRes {}


pub struct Empty;
impl ros2_client::Service for Empty {
    type Request = EmptyReq;
    type Response = EmptyRes;

    fn request_type_name(&self) -> &str { "EmptyReq" }
    fn response_type_name(&self) -> &str { "EmptyRes" }
}
