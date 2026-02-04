use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoJSONProperties {
    pub id: u32,
    #[serde(rename = "type")]    pub type_: u32,
    pub enflation_radius: Vec<f64>,
    pub bondary_radius: Vec<f64>,
}

impl Default for GeoJSONProperties {
    fn default() -> Self {
        GeoJSONProperties {
            id: 0,
            type_: 0,
            enflation_radius: Vec::new(),
            bondary_radius: Vec::new(),
        }
    }
}

impl ros2_client::Message for GeoJSONProperties {}
