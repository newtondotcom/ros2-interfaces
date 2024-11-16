use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLoadCarrierReq {
    pub load_carrier: crate::rc_reason_msgs::msg::LoadCarrierModel,
}

impl Default for SetLoadCarrierReq {
    fn default() -> Self {
        SetLoadCarrierReq {
            load_carrier: crate::rc_reason_msgs::msg::LoadCarrierModel::default(),
        }
    }
}

impl ros2_client::Message for SetLoadCarrierReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLoadCarrierRes {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for SetLoadCarrierRes {
    fn default() -> Self {
        SetLoadCarrierRes {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for SetLoadCarrierRes {}


pub struct SetLoadCarrier;
impl ros2_client::Service for SetLoadCarrier {
    type Request = SetLoadCarrierReq;
    type Response = SetLoadCarrierRes;

    fn request_type_name(&self) -> &str { "SetLoadCarrierReq" }
    fn response_type_name(&self) -> &str { "SetLoadCarrierRes" }
}
