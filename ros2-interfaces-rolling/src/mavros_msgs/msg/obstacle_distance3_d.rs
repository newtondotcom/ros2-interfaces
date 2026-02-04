use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObstacleDistance3D {
    pub header: crate::std_msgs::msg::Header,
    pub sensor_type: u8,
    pub frame: u8,
    pub obstacle_id: u16,
    pub position: crate::geometry_msgs::msg::Point,
    pub min_distance: f32,
    pub max_distance: f32,
}

impl Default for ObstacleDistance3D {
    fn default() -> Self {
        ObstacleDistance3D {
            header: crate::std_msgs::msg::Header::default(),
            sensor_type: 0,
            frame: 0,
            obstacle_id: 0,
            position: crate::geometry_msgs::msg::Point::default(),
            min_distance: 0.0,
            max_distance: 0.0,
        }
    }
}

impl ros2_client::Message for ObstacleDistance3D {}
