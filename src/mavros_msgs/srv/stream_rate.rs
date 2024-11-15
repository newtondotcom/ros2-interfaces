use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamRateReq {
    pub stream_id: u8,
    pub message_rate: u16,
    pub on_off: bool,
}

impl StreamRateReq {
    pub const STREAM_ALL: u8 = 0;
    pub const STREAM_RAW_SENSORS: u8 = 1;
    pub const STREAM_EXTENDED_STATUS: u8 = 2;
    pub const STREAM_RC_CHANNELS: u8 = 3;
    pub const STREAM_RAW_CONTROLLER: u8 = 4;
    pub const STREAM_POSITION: u8 = 6;
    pub const STREAM_EXTRA1: u8 = 10;
    pub const STREAM_EXTRA2: u8 = 11;
    pub const STREAM_EXTRA3: u8 = 12;
}

impl Default for StreamRateReq {
    fn default() -> Self {
        StreamRateReq {
            stream_id: 0,
            message_rate: 0,
            on_off: false,
        }
    }
}

impl ros2_client::Message for StreamRateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamRateRes {

}

impl Default for StreamRateRes {
    fn default() -> Self {
        StreamRateRes {

        }
    }
}

impl ros2_client::Message for StreamRateRes {}


pub struct StreamRate;
impl ros2_client::Service for StreamRate {
    type Request = StreamRateReq;
    type Response = StreamRateRes;

    fn request_type_name(&self) -> &str { "StreamRateReq" }
    fn response_type_name(&self) -> &str { "StreamRateRes" }
}
