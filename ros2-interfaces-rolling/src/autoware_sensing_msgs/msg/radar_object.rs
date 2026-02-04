use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarObject {
    pub object_id: u32,
    pub age: u16,
    pub measurement_status: u8,
    pub movement_status: u8,
    pub position: crate::geometry_msgs::msg::Point,
    pub velocity: crate::geometry_msgs::msg::Vector3,
    pub acceleration: crate::geometry_msgs::msg::Vector3,
    pub size: crate::geometry_msgs::msg::Vector3,
    pub position_covariance: [f32; 6],
    pub velocity_covariance: [f32; 6],
    pub acceleration_covariance: [f32; 6],
    pub size_covariance: [f32; 6],
    pub orientation: f32,
    pub orientation_std: f32,
    pub orientation_rate: f32,
    pub orientation_rate_std: f32,
    pub existence_probability: f32,
    pub classifications: Vec<crate::autoware_sensing_msgs::msg::RadarClassification>,
}

impl RadarObject {
    pub const INVALID_COV_VALUE: f32 = -1.0;
    pub const MEASUREMENT_STATUS_INVALID: u8 = 0;
    pub const MEASUREMENT_STATUS_MEASURED: u8 = 1;
    pub const MEASUREMENT_STATUS_PREDICTED: u8 = 2;
    pub const MEASUREMENT_STATUS_NEW: u8 = 3;
    pub const MEASUREMENT_STATUS_UNKNOWN: u8 = 4;
    pub const MOVEMENT_STATUS_INVALID: u8 = 0;
    pub const MOVEMENT_STATUS_DYNAMIC: u8 = 1;
    pub const MOVEMENT_STATUS_STATIC: u8 = 2;
    pub const MOVEMENT_STATUS_UNKNOWN: u8 = 3;
}

impl Default for RadarObject {
    fn default() -> Self {
        RadarObject {
            object_id: 0,
            age: 0,
            measurement_status: 0,
            movement_status: 0,
            position: crate::geometry_msgs::msg::Point::default(),
            velocity: crate::geometry_msgs::msg::Vector3::default(),
            acceleration: crate::geometry_msgs::msg::Vector3::default(),
            size: crate::geometry_msgs::msg::Vector3::default(),
            position_covariance: [0.0; 6],
            velocity_covariance: [0.0; 6],
            acceleration_covariance: [0.0; 6],
            size_covariance: [0.0; 6],
            orientation: 0.0,
            orientation_std: 0.0,
            orientation_rate: 0.0,
            orientation_rate_std: 0.0,
            existence_probability: 0.0,
            classifications: Vec::new(),
        }
    }
}

impl ros2_client::Message for RadarObject {}
