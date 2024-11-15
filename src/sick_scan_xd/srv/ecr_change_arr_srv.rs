use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ECRChangeArrSrvReq {
    pub active: bool,
}

impl Default for ECRChangeArrSrvReq {
    fn default() -> Self {
        ECRChangeArrSrvReq {
            active: false,
        }
    }
}

impl ros2_client::Message for ECRChangeArrSrvReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ECRChangeArrSrvRes {
    pub success: bool,
}

impl Default for ECRChangeArrSrvRes {
    fn default() -> Self {
        ECRChangeArrSrvRes {
            success: false,
        }
    }
}

impl ros2_client::Message for ECRChangeArrSrvRes {}


pub struct ECRChangeArrSrv;
impl ros2_client::Service for ECRChangeArrSrv {
    type Request = ECRChangeArrSrvReq;
    type Response = ECRChangeArrSrvRes;

    fn request_type_name(&self) -> &str { "ECRChangeArrSrvReq" }
    fn response_type_name(&self) -> &str { "ECRChangeArrSrvRes" }
}
