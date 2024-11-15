use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarmStartReq {
    pub reset_type: u8,
}

impl Default for WarmStartReq {
    fn default() -> Self {
        WarmStartReq {
            reset_type: 0,
        }
    }
}

impl ros2_client::Message for WarmStartReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarmStartRes {

}

impl Default for WarmStartRes {
    fn default() -> Self {
        WarmStartRes {

        }
    }
}

impl ros2_client::Message for WarmStartRes {}


pub struct WarmStart;
impl ros2_client::Service for WarmStart {
    type Request = WarmStartReq;
    type Response = WarmStartRes;

    fn request_type_name(&self) -> &str { "WarmStartReq" }
    fn response_type_name(&self) -> &str { "WarmStartRes" }
}
