use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenDroneIDSelfID {
    pub header: crate::std_msgs::msg::Header,
    pub id_or_mac: ::std::string::String,
    pub description_type: u8,
    pub description: ::std::string::String,
}

impl OpenDroneIDSelfID {
    pub const DESC_TYPE_TEXT: u8 = 0;
    pub const DESC_TYPE_EMERGENCY: u8 = 1;
    pub const DESC_TYPE_EXTENDED_STATUS: u8 = 2;
}

impl Default for OpenDroneIDSelfID {
    fn default() -> Self {
        OpenDroneIDSelfID {
            header: crate::std_msgs::msg::Header::default(),
            id_or_mac: ::std::string::String::new(),
            description_type: 0,
            description: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for OpenDroneIDSelfID {}
