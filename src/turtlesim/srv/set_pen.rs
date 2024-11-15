use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPenReq {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub width: u8,
    pub off: u8,
}

impl Default for SetPenReq {
    fn default() -> Self {
        SetPenReq {
            r: 0,
            g: 0,
            b: 0,
            width: 0,
            off: 0,
        }
    }
}

impl ros2_client::Message for SetPenReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPenRes {

}

impl Default for SetPenRes {
    fn default() -> Self {
        SetPenRes {

        }
    }
}

impl ros2_client::Message for SetPenRes {}


pub struct SetPen;
impl ros2_client::Service for SetPen {
    type Request = SetPenReq;
    type Response = SetPenRes;

    fn request_type_name(&self) -> &str { "SetPenReq" }
    fn response_type_name(&self) -> &str { "SetPenRes" }
}
