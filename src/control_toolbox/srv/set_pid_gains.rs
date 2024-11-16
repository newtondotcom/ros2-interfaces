use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPidGainsReq {
    pub p: f64,
    pub i: f64,
    pub d: f64,
    pub i_clamp: f64,
    pub antiwindup: bool,
}

impl Default for SetPidGainsReq {
    fn default() -> Self {
        SetPidGainsReq {
            p: 0.0,
            i: 0.0,
            d: 0.0,
            i_clamp: 0.0,
            antiwindup: false,
        }
    }
}

impl ros2_client::Message for SetPidGainsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPidGainsRes {

}

impl Default for SetPidGainsRes {
    fn default() -> Self {
        SetPidGainsRes {

        }
    }
}

impl ros2_client::Message for SetPidGainsRes {}


pub struct SetPidGains;
impl ros2_client::Service for SetPidGains {
    type Request = SetPidGainsReq;
    type Response = SetPidGainsRes;

    fn request_type_name(&self) -> &str { "SetPidGainsReq" }
    fn response_type_name(&self) -> &str { "SetPidGainsRes" }
}
