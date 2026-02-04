use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Azimuth {
    pub header: crate::std_msgs::msg::Header,
    pub azimuth: f64,
    pub variance: f64,
    pub unit: u8,
    pub orientation: u8,
    pub reference: u8,
}

impl Azimuth {
    pub const UNIT_RAD: u8 = 0;
    pub const UNIT_DEG: u8 = 1;
    pub const ORIENTATION_ENU: u8 = 0;
    pub const ORIENTATION_NED: u8 = 1;
    pub const REFERENCE_MAGNETIC: u8 = 0;
    pub const REFERENCE_GEOGRAPHIC: u8 = 1;
    pub const REFERENCE_UTM: u8 = 2;
}

impl Default for Azimuth {
    fn default() -> Self {
        Azimuth {
            header: crate::std_msgs::msg::Header::default(),
            azimuth: 0.0,
            variance: 0.0,
            unit: 0,
            orientation: 0,
            reference: 0,
        }
    }
}

impl ros2_client::Message for Azimuth {}
