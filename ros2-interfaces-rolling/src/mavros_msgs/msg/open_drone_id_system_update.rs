use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenDroneIDSystemUpdate {
    pub header: crate::std_msgs::msg::Header,
    pub operator_latitude: i32,
    pub operator_longitude: i32,
    pub operator_altitude_geo: f32,
}

impl Default for OpenDroneIDSystemUpdate {
    fn default() -> Self {
        OpenDroneIDSystemUpdate {
            header: crate::std_msgs::msg::Header::default(),
            operator_latitude: 0,
            operator_longitude: 0,
            operator_altitude_geo: 0.0,
        }
    }
}

impl ros2_client::Message for OpenDroneIDSystemUpdate {}
