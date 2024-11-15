use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCsoftresetSrvReq {

}

impl Default for SCsoftresetSrvReq {
    fn default() -> Self {
        SCsoftresetSrvReq {

        }
    }
}

impl ros2_client::Message for SCsoftresetSrvReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCsoftresetSrvRes {
    pub success: bool,
}

impl Default for SCsoftresetSrvRes {
    fn default() -> Self {
        SCsoftresetSrvRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SCsoftresetSrvRes {}


pub struct SCsoftresetSrv;
impl ros2_client::Service for SCsoftresetSrv {
    type Request = SCsoftresetSrvReq;
    type Response = SCsoftresetSrvRes;

    fn request_type_name(&self) -> &str { "SCsoftresetSrvReq" }
    fn response_type_name(&self) -> &str { "SCsoftresetSrvRes" }
}
