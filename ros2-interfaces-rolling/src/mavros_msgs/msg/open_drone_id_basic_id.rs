use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenDroneIDBasicID {
    pub header: crate::std_msgs::msg::Header,
    pub id_or_mac: ::std::string::String,
    pub id_type: u8,
    pub ua_type: u8,
    pub uas_id: ::std::string::String,
}

impl OpenDroneIDBasicID {
    pub const ID_TYPE_NONE: u8 = 0;
    pub const ID_TYPE_SERIAL_NUMBER: u8 = 1;
    pub const ID_TYPE_CAA_REGISTRATION_ID: u8 = 2;
    pub const ID_TYPE_UTM_ASSIGNED_UUID: u8 = 3;
    pub const ID_TYPE_SPECIFIC_SESSION_ID: u8 = 4;
    pub const UA_TYPE_NONE: u8 = 0;
    pub const UA_TYPE_AEROPLANE: u8 = 1;
    pub const UA_TYPE_HELICOPTER_OR_MULTIROTOR: u8 = 2;
    pub const UA_TYPE_GYROPLANE: u8 = 3;
    pub const UA_TYPE_HYBRID_LIFT: u8 = 4;
    pub const UA_TYPE_ORNITHOPTER: u8 = 5;
    pub const UA_TYPE_GLIDER: u8 = 6;
    pub const UA_TYPE_KITE: u8 = 7;
    pub const UA_TYPE_FREE_BALLOON: u8 = 8;
    pub const UA_TYPE_CAPTIVE_BALLOON: u8 = 9;
    pub const UA_TYPE_AIRSHIP: u8 = 10;
    pub const UA_TYPE_FREE_FALL_PARACHUTE: u8 = 11;
    pub const UA_TYPE_ROCKET: u8 = 12;
    pub const UA_TYPE_TETHERED_POWERED_AIRCRAFT: u8 = 13;
    pub const UA_TYPE_GROUND_OBSTACLE: u8 = 14;
    pub const UA_TYPE_OTHER: u8 = 15;
}

impl Default for OpenDroneIDBasicID {
    fn default() -> Self {
        OpenDroneIDBasicID {
            header: crate::std_msgs::msg::Header::default(),
            id_or_mac: ::std::string::String::new(),
            id_type: 0,
            ua_type: 0,
            uas_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for OpenDroneIDBasicID {}
