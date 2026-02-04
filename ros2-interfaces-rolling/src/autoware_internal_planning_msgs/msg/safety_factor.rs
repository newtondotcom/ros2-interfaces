use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyFactor {
    #[serde(rename = "type")]    pub type_: u16,
    pub object_id: crate::unique_identifier_msgs::msg::UUID,
    pub predicted_path: crate::autoware_perception_msgs::msg::PredictedPath,
    pub ttc_begin: f32,
    pub ttc_end: f32,
    pub points: Vec<crate::geometry_msgs::msg::Point>,
    pub is_safe: bool,
}

impl SafetyFactor {
    pub const UNKNOWN: u16 = 0;
    pub const POINTCLOUD: u16 = 1;
    pub const OBJECT: u16 = 2;
}

impl Default for SafetyFactor {
    fn default() -> Self {
        SafetyFactor {
            type_: 0,
            object_id: crate::unique_identifier_msgs::msg::UUID::default(),
            predicted_path: crate::autoware_perception_msgs::msg::PredictedPath::default(),
            ttc_begin: 0.0,
            ttc_end: 0.0,
            points: Vec::new(),
            is_safe: false,
        }
    }
}

impl ros2_client::Message for SafetyFactor {}
