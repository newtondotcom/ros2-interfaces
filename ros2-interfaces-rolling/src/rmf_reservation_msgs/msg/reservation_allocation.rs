use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReservationAllocation {
    pub ticket: crate::rmf_reservation_msgs::msg::Ticket,
    pub instruction_type: u8,
    pub chosen_alternative: u64,
    pub resource: ::std::string::String,
}

impl ReservationAllocation {
    pub const IMMEDIATELY_PROCEED: u8 = 0;
    pub const WAIT_IDENTIFIED: u8 = 1;
}

impl Default for ReservationAllocation {
    fn default() -> Self {
        ReservationAllocation {
            ticket: crate::rmf_reservation_msgs::msg::Ticket::default(),
            instruction_type: 0,
            chosen_alternative: 0,
            resource: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ReservationAllocation {}
