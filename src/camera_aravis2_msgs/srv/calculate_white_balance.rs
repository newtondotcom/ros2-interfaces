use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculateWhiteBalanceReq {

}

impl Default for CalculateWhiteBalanceReq {
    fn default() -> Self {
        CalculateWhiteBalanceReq {

        }
    }
}

impl ros2_client::Message for CalculateWhiteBalanceReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculateWhiteBalanceRes {
    pub is_successful: bool,
    pub balance_ratios: Vec<crate::diagnostic_msgs::msg::KeyValue>,
}

impl Default for CalculateWhiteBalanceRes {
    fn default() -> Self {
        CalculateWhiteBalanceRes {
            is_successful: false,
            balance_ratios: Vec::new(),
        }
    }
}

impl ros2_client::Message for CalculateWhiteBalanceRes {}


pub struct CalculateWhiteBalance;
impl ros2_client::Service for CalculateWhiteBalance {
    type Request = CalculateWhiteBalanceReq;
    type Response = CalculateWhiteBalanceRes;

    fn request_type_name(&self) -> &str { "CalculateWhiteBalanceReq" }
    fn response_type_name(&self) -> &str { "CalculateWhiteBalanceRes" }
}
