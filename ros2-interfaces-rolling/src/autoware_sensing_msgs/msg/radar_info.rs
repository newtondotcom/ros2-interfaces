use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarInfo {
    pub header: crate::std_msgs::msg::Header,
    pub object_fields_info: Vec<crate::autoware_sensing_msgs::msg::RadarFieldInfo>,
    pub detection_fields_info: Vec<crate::autoware_sensing_msgs::msg::RadarFieldInfo>,
    pub available_classes: Vec<u32>,
    pub absolute_dynamics: bool,
}

impl Default for RadarInfo {
    fn default() -> Self {
        RadarInfo {
            header: crate::std_msgs::msg::Header::default(),
            object_fields_info: Vec::new(),
            detection_fields_info: Vec::new(),
            available_classes: Vec::new(),
            absolute_dynamics: false,
        }
    }
}

impl ros2_client::Message for RadarInfo {}
