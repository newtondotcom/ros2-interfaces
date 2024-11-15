use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCrebootSrvReq {

}

impl Default for SCrebootSrvReq {
    fn default() -> Self {
        SCrebootSrvReq {

        }
    }
}

impl ros2_client::Message for SCrebootSrvReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCrebootSrvRes {
    pub success: bool,
}

impl Default for SCrebootSrvRes {
    fn default() -> Self {
        SCrebootSrvRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SCrebootSrvRes {}


pub struct SCrebootSrv;
impl ros2_client::Service for SCrebootSrv {
    type Request = SCrebootSrvReq;
    type Response = SCrebootSrvRes;

    fn request_type_name(&self) -> &str { "SCrebootSrvReq" }
    fn response_type_name(&self) -> &str { "SCrebootSrvRes" }
}
