use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ticket {
    pub header: crate::rmf_reservation_msgs::msg::RequestHeader,
    pub ticket_id: u64,
}

impl Default for Ticket {
    fn default() -> Self {
        Ticket {
            header: crate::rmf_reservation_msgs::msg::RequestHeader::default(),
            ticket_id: 0,
        }
    }
}

impl ros2_client::Message for Ticket {}
