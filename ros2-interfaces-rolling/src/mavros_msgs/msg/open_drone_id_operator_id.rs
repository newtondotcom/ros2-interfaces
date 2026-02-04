use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenDroneIDOperatorID {
    pub header: crate::std_msgs::msg::Header,
    pub id_or_mac: ::std::string::String,
    pub operator_id_type: u8,
    pub operator_id: ::std::string::String,
}

impl OpenDroneIDOperatorID {
    pub const ID_TYPE_CAA: u8 = 0;
}

impl Default for OpenDroneIDOperatorID {
    fn default() -> Self {
        OpenDroneIDOperatorID {
            header: crate::std_msgs::msg::Header::default(),
            id_or_mac: ::std::string::String::new(),
            operator_id_type: 0,
            operator_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for OpenDroneIDOperatorID {}
