use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenDroneIDSystem {
    pub header: crate::std_msgs::msg::Header,
    pub id_or_mac: ::std::string::String,
    pub operator_location_type: u8,
    pub classification_type: u8,
    pub operator_latitude: i32,
    pub operator_longitude: i32,
    pub area_count: u16,
    pub area_radius: u16,
    pub area_ceiling: f32,
    pub area_floor: f32,
    pub category_eu: u8,
    pub class_eu: u8,
    pub operator_altitude_geo: f32,
}

impl OpenDroneIDSystem {
    pub const LOCATION_TYPE_TAKEOFF: u8 = 0;
    pub const LOCATION_TYPE_LIVE_GNSS: u8 = 1;
    pub const LOCATION_TYPE_FIXED: u8 = 2;
    pub const CLASSIFICATION_TYPE_UNDECLARED: u8 = 0;
    pub const CLASSIFICATION_TYPE_EU: u8 = 1;
}

impl Default for OpenDroneIDSystem {
    fn default() -> Self {
        OpenDroneIDSystem {
            header: crate::std_msgs::msg::Header::default(),
            id_or_mac: ::std::string::String::new(),
            operator_location_type: 0,
            classification_type: 0,
            operator_latitude: 0,
            operator_longitude: 0,
            area_count: 0,
            area_radius: 0,
            area_ceiling: 0.0,
            area_floor: 0.0,
            category_eu: 0,
            class_eu: 0,
            operator_altitude_geo: 0.0,
        }
    }
}

impl ros2_client::Message for OpenDroneIDSystem {}
