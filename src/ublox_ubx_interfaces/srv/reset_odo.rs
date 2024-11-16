use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetODOReq {

}

impl Default for ResetODOReq {
    fn default() -> Self {
        ResetODOReq {

        }
    }
}

impl ros2_client::Message for ResetODOReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetODORes {

}

impl Default for ResetODORes {
    fn default() -> Self {
        ResetODORes {

        }
    }
}

impl ros2_client::Message for ResetODORes {}


pub struct ResetODO;
impl ros2_client::Service for ResetODO {
    type Request = ResetODOReq;
    type Response = ResetODORes;

    fn request_type_name(&self) -> &str { "ResetODOReq" }
    fn response_type_name(&self) -> &str { "ResetODORes" }
}
