use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetExposureReq {
    pub auto_exposure: bool,
    pub time: i64,
}

impl Default for SetExposureReq {
    fn default() -> Self {
        SetExposureReq {
            auto_exposure: false,
            time: 0,
        }
    }
}

impl ros2_client::Message for SetExposureReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetExposureRes {
    pub auto_exposure: bool,
    pub time: i64,
}

impl Default for SetExposureRes {
    fn default() -> Self {
        SetExposureRes {
            auto_exposure: false,
            time: 0,
        }
    }
}

impl ros2_client::Message for SetExposureRes {}


pub struct SetExposure;
impl ros2_client::Service for SetExposure {
    type Request = SetExposureReq;
    type Response = SetExposureRes;

    fn request_type_name(&self) -> &str { "SetExposureReq" }
    fn response_type_name(&self) -> &str { "SetExposureRes" }
}
