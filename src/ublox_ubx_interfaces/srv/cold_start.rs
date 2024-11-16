use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColdStartReq {
    pub reset_type: u8,
}

impl ColdStartReq {
    pub const HW_RESET_IMMEDIATELY: u8 = 0x00;
    pub const SW_RESET_CONTROLLED: u8 = 0x01;
    pub const SW_RESET_CONTROLLED_GNSS: u8 = 0x02;
    pub const HW_RESET_AFTER_SHUTDOWN: u8 = 0x04;
    pub const GNSS_STOP_CONTROLLED: u8 = 0x08;
    pub const GNSS_START_CONTROLLED: u8 = 0x09;
}

impl Default for ColdStartReq {
    fn default() -> Self {
        ColdStartReq {
            reset_type: 0,
        }
    }
}

impl ros2_client::Message for ColdStartReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColdStartRes {

}

impl Default for ColdStartRes {
    fn default() -> Self {
        ColdStartRes {

        }
    }
}

impl ros2_client::Message for ColdStartRes {}


pub struct ColdStart;
impl ros2_client::Service for ColdStart {
    type Request = ColdStartReq;
    type Response = ColdStartRes;

    fn request_type_name(&self) -> &str { "ColdStartReq" }
    fn response_type_name(&self) -> &str { "ColdStartRes" }
}
