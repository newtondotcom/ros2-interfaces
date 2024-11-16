use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseRequest {
    pub ticket: crate::rmf_reservation_msgs::msg::Ticket,
    pub location: ::std::string::String,
}

impl Default for ReleaseRequest {
    fn default() -> Self {
        ReleaseRequest {
            ticket: crate::rmf_reservation_msgs::msg::Ticket::default(),
            location: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ReleaseRequest {}
