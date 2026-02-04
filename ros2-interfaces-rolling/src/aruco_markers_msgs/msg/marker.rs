use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marker {
    pub header: crate::std_msgs::msg::Header,
    pub id: u32,
    pub pose: crate::geometry_msgs::msg::PoseStamped,
    pub pixel_x: f64, // default: 0
    pub pixel_y: f64, // default: 0
}

impl Default for Marker {
    fn default() -> Self {
        Marker {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            pose: crate::geometry_msgs::msg::PoseStamped::default(),
            pixel_x: 0.0,
            pixel_y: 0.0,
        }
    }
}

impl ros2_client::Message for Marker {}
