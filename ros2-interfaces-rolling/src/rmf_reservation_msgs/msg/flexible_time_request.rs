use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlexibleTimeRequest {
    pub header: crate::rmf_reservation_msgs::msg::RequestHeader,
    pub alternatives: Vec<crate::rmf_reservation_msgs::msg::FlexibleTimeReservationAlt>,
}

impl Default for FlexibleTimeRequest {
    fn default() -> Self {
        FlexibleTimeRequest {
            header: crate::rmf_reservation_msgs::msg::RequestHeader::default(),
            alternatives: Vec::new(),
        }
    }
}

impl ros2_client::Message for FlexibleTimeRequest {}
