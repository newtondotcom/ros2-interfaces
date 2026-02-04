use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoJSONFeature {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub geometry: crate::tuw_object_map_msgs::msg::GeoJSONGeometry,
}

impl Default for GeoJSONFeature {
    fn default() -> Self {
        GeoJSONFeature {
            type_: ::std::string::String::new(),
            geometry: crate::tuw_object_map_msgs::msg::GeoJSONGeometry::default(),
        }
    }
}

impl ros2_client::Message for GeoJSONFeature {}
