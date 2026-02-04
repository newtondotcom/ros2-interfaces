use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorStateInformation {
    pub header: crate::std_msgs::msg::Header,
    pub sensor_state: u8,
}

impl Default for SensorStateInformation {
    fn default() -> Self {
        SensorStateInformation {
            header: crate::std_msgs::msg::Header::default(),
            sensor_state: 0,
        }
    }
}

impl ros2_client::Message for SensorStateInformation {}
