use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TexturedBox {
    pub dimensions: crate::rc_reason_msgs::msg::Box,
    pub dimensions_tolerance_m: f64,
    pub max_deformation_m: f64,
    pub template_id: ::std::string::String,
}

impl Default for TexturedBox {
    fn default() -> Self {
        TexturedBox {
            dimensions: crate::rc_reason_msgs::msg::Box::default(),
            dimensions_tolerance_m: 0.0,
            max_deformation_m: 0.0,
            template_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for TexturedBox {}
