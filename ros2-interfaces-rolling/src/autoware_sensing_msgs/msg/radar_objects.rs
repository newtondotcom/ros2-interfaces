use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarObjects {
    pub header: crate::std_msgs::msg::Header,
    pub objects: Vec<crate::autoware_sensing_msgs::msg::RadarObject>,
}

impl Default for RadarObjects {
    fn default() -> Self {
        RadarObjects {
            header: crate::std_msgs::msg::Header::default(),
            objects: Vec::new(),
        }
    }
}

impl ros2_client::Message for RadarObjects {}
