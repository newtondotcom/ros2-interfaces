use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotStartReq {
    pub reset_type: u8,
}

impl Default for HotStartReq {
    fn default() -> Self {
        HotStartReq {
            reset_type: 0,
        }
    }
}

impl ros2_client::Message for HotStartReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotStartRes {

}

impl Default for HotStartRes {
    fn default() -> Self {
        HotStartRes {

        }
    }
}

impl ros2_client::Message for HotStartRes {}


pub struct HotStart;
impl ros2_client::Service for HotStart {
    type Request = HotStartReq;
    type Response = HotStartRes;

    fn request_type_name(&self) -> &str { "HotStartReq" }
    fn response_type_name(&self) -> &str { "HotStartRes" }
}
