use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContaminationResultSrvReq {

}

impl Default for GetContaminationResultSrvReq {
    fn default() -> Self {
        GetContaminationResultSrvReq {

        }
    }
}

impl ros2_client::Message for GetContaminationResultSrvReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContaminationResultSrvRes {
    pub warning: u8,
    pub error: u8,
    pub success: bool,
}

impl Default for GetContaminationResultSrvRes {
    fn default() -> Self {
        GetContaminationResultSrvRes {
            warning: 0,
            error: 0,
            success: false,
        }
    }
}

impl ros2_client::Message for GetContaminationResultSrvRes {}


pub struct GetContaminationResultSrv;
impl ros2_client::Service for GetContaminationResultSrv {
    type Request = GetContaminationResultSrvReq;
    type Response = GetContaminationResultSrvRes;

    fn request_type_name(&self) -> &str { "GetContaminationResultSrvReq" }
    fn response_type_name(&self) -> &str { "GetContaminationResultSrvRes" }
}
