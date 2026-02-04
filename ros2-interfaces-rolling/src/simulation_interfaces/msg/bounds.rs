use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bounds {
    #[serde(rename = "type")]    pub type_: u8,
    pub points: Vec<crate::geometry_msgs::msg::Vector3>,
}

impl Bounds {
    pub const TYPE_EMPTY: u8 = 0;
    pub const TYPE_BOX: u8 = 1;
    pub const TYPE_CONVEX_HULL: u8 = 2;
    pub const TYPE_SPHERE: u8 = 3;
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds {
            type_: 0,
            points: Vec::new(),
        }
    }
}

impl ros2_client::Message for Bounds {}
