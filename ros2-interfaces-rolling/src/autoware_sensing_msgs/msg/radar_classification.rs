use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarClassification {
    pub label: u8,
    pub probability: f32,
}

impl RadarClassification {
    pub const UNKNOWN: u8 = 0;
    pub const CAR: u8 = 1;
    pub const TRUCK: u8 = 2;
    pub const BUS: u8 = 3;
    pub const TRAILER: u8 = 4;
    pub const MOTORCYCLE: u8 = 5;
    pub const BICYCLE: u8 = 6;
    pub const PEDESTRIAN: u8 = 7;
    pub const ANIMAL: u8 = 8;
    pub const HAZARD: u8 = 9;
    pub const OVER_DRIVABLE: u8 = 10;
    pub const UNDER_DRIVABLE: u8 = 11;
}

impl Default for RadarClassification {
    fn default() -> Self {
        RadarClassification {
            label: 0,
            probability: 0.0,
        }
    }
}

impl ros2_client::Message for RadarClassification {}
