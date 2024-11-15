use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SickScanExitSrvReq {

}

impl Default for SickScanExitSrvReq {
    fn default() -> Self {
        SickScanExitSrvReq {

        }
    }
}

impl ros2_client::Message for SickScanExitSrvReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SickScanExitSrvRes {
    pub success: bool,
}

impl Default for SickScanExitSrvRes {
    fn default() -> Self {
        SickScanExitSrvRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SickScanExitSrvRes {}


pub struct SickScanExitSrv;
impl ros2_client::Service for SickScanExitSrv {
    type Request = SickScanExitSrvReq;
    type Response = SickScanExitSrvRes;

    fn request_type_name(&self) -> &str { "SickScanExitSrvReq" }
    fn response_type_name(&self) -> &str { "SickScanExitSrvRes" }
}
