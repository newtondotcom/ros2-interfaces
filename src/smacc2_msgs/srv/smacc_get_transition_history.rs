use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccGetTransitionHistoryReq {

}

impl Default for SmaccGetTransitionHistoryReq {
    fn default() -> Self {
        SmaccGetTransitionHistoryReq {

        }
    }
}

impl ros2_client::Message for SmaccGetTransitionHistoryReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccGetTransitionHistoryRes {
    pub history: Vec<crate::smacc2_msgs::msg::SmaccTransitionLogEntry>,
}

impl Default for SmaccGetTransitionHistoryRes {
    fn default() -> Self {
        SmaccGetTransitionHistoryRes {
            history: Vec::new(),
        }
    }
}

impl ros2_client::Message for SmaccGetTransitionHistoryRes {}


pub struct SmaccGetTransitionHistory;
impl ros2_client::Service for SmaccGetTransitionHistory {
    type Request = SmaccGetTransitionHistoryReq;
    type Response = SmaccGetTransitionHistoryRes;

    fn request_type_name(&self) -> &str { "SmaccGetTransitionHistoryReq" }
    fn response_type_name(&self) -> &str { "SmaccGetTransitionHistoryRes" }
}
