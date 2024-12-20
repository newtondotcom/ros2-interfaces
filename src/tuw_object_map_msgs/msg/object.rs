use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object {
    pub id: i64,
    #[serde(rename = "type")]    pub type_: u32,
    pub geo_points: Vec<crate::tuw_object_map_msgs::msg::GeoPoint>,
    pub map_points: Vec<crate::geometry_msgs::msg::Point>,
    pub enflation_radius: Vec<f64>,
    pub bondary_radius: Vec<f64>,
}

impl Object {
    pub const TYPE_NA: u32 = 0;
    pub const TYPE_FRAME_ORIGIN: u32 = 100;
    pub const TYPE_FRAME_WINDOW: u32 = 101;
    pub const TYPE_PLANT: u32 = 200;
    pub const TYPE_PLANT_WINE_ROW: u32 = 201;
    pub const TYPE_OBSTACLE: u32 = 300;
    pub const TYPE_OBSTACLE_HOUSE: u32 = 301;
    pub const TYPE_OBSTACLE_TREE: u32 = 302;
    pub const TYPE_TRANSIT: u32 = 400;
    pub const TYPE_TRANSIT_STREET: u32 = 401;
    pub const TYPE_TRANSIT_GRAVEL: u32 = 402;
}

impl Default for Object {
    fn default() -> Self {
        Object {
            id: 0,
            type_: 0,
            geo_points: Vec::new(),
            map_points: Vec::new(),
            enflation_radius: Vec::new(),
            bondary_radius: Vec::new(),
        }
    }
}

impl ros2_client::Message for Object {}
