use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoadCarriersReq {
    pub load_carrier_ids: Vec<::std::string::String>,
}

impl Default for GetLoadCarriersReq {
    fn default() -> Self {
        GetLoadCarriersReq {
            load_carrier_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetLoadCarriersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoadCarriersRes {
    pub load_carriers: Vec<crate::rc_reason_msgs::msg::LoadCarrierModel>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for GetLoadCarriersRes {
    fn default() -> Self {
        GetLoadCarriersRes {
            load_carriers: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for GetLoadCarriersRes {}


pub struct GetLoadCarriers;
impl ros2_client::Service for GetLoadCarriers {
    type Request = GetLoadCarriersReq;
    type Response = GetLoadCarriersRes;

    fn request_type_name(&self) -> &str { "GetLoadCarriersReq" }
    fn response_type_name(&self) -> &str { "GetLoadCarriersRes" }
}
