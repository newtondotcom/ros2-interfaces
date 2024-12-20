use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoJSON {
    pub header: crate::std_msgs::msg::Header,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub features: Vec<crate::tuw_object_map_msgs::msg::GeoJSONFeature>,
}

impl Default for GeoJSON {
    fn default() -> Self {
        GeoJSON {
            header: crate::std_msgs::msg::Header::default(),
            type_: ::std::string::String::new(),
            features: Vec::new(),
        }
    }
}

impl ros2_client::Message for GeoJSON {}
