use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCdevicestateSrvReq {

}

impl Default for SCdevicestateSrvReq {
    fn default() -> Self {
        SCdevicestateSrvReq {

        }
    }
}

impl ros2_client::Message for SCdevicestateSrvReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCdevicestateSrvRes {
    pub state: i32,
    pub success: bool,
}

impl Default for SCdevicestateSrvRes {
    fn default() -> Self {
        SCdevicestateSrvRes {
            state: 0,
            success: false,
        }
    }
}

impl ros2_client::Message for SCdevicestateSrvRes {}


pub struct SCdevicestateSrv;
impl ros2_client::Service for SCdevicestateSrv {
    type Request = SCdevicestateSrvReq;
    type Response = SCdevicestateSrvRes;

    fn request_type_name(&self) -> &str { "SCdevicestateSrvReq" }
    fn response_type_name(&self) -> &str { "SCdevicestateSrvRes" }
}
