use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoJSONGeometry {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub coordinates: Vec<crate::tuw_object_map_msgs::msg::GeoPoint>,
    pub properties: crate::tuw_object_map_msgs::msg::GeoJSONProperties,
}

impl Default for GeoJSONGeometry {
    fn default() -> Self {
        GeoJSONGeometry {
            type_: ::std::string::String::new(),
            coordinates: Vec::new(),
            properties: crate::tuw_object_map_msgs::msg::GeoJSONProperties::default(),
        }
    }
}

impl ros2_client::Message for GeoJSONGeometry {}
