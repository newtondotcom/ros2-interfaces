use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaimRequest {
    pub ticket: crate::rmf_reservation_msgs::msg::Ticket,
    pub wait_points: Vec<::std::string::String>,
}

impl Default for ClaimRequest {
    fn default() -> Self {
        ClaimRequest {
            ticket: crate::rmf_reservation_msgs::msg::Ticket::default(),
            wait_points: Vec::new(),
        }
    }
}

impl ros2_client::Message for ClaimRequest {}
