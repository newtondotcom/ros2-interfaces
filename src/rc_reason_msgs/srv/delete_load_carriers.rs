use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteLoadCarriersReq {
    pub load_carrier_ids: Vec<::std::string::String>,
}

impl Default for DeleteLoadCarriersReq {
    fn default() -> Self {
        DeleteLoadCarriersReq {
            load_carrier_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for DeleteLoadCarriersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteLoadCarriersRes {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteLoadCarriersRes {
    fn default() -> Self {
        DeleteLoadCarriersRes {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for DeleteLoadCarriersRes {}


pub struct DeleteLoadCarriers;
impl ros2_client::Service for DeleteLoadCarriers {
    type Request = DeleteLoadCarriersReq;
    type Response = DeleteLoadCarriersRes;

    fn request_type_name(&self) -> &str { "DeleteLoadCarriersReq" }
    fn response_type_name(&self) -> &str { "DeleteLoadCarriersRes" }
}
