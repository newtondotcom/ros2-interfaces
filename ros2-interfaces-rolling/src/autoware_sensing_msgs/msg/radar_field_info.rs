use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarFieldInfo {
    pub field_name: crate::std_msgs::msg::String,
    pub min_value_available: bool,
    pub max_value_available: bool,
    pub resolution_available: bool,
    pub min_value: f32,
    pub max_value: f32,
    pub resolution: f32,
}

impl Default for RadarFieldInfo {
    fn default() -> Self {
        RadarFieldInfo {
            field_name: crate::std_msgs::msg::String::default(),
            min_value_available: false,
            max_value_available: false,
            resolution_available: false,
            min_value: 0.0,
            max_value: 0.0,
            resolution: 0.0,
        }
    }
}

impl ros2_client::Message for RadarFieldInfo {}
