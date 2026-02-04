use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratorInfo {
    pub generator_id: crate::unique_identifier_msgs::msg::UUID,
    pub generator_name: crate::std_msgs::msg::String,
}

impl Default for GeneratorInfo {
    fn default() -> Self {
        GeneratorInfo {
            generator_id: crate::unique_identifier_msgs::msg::UUID::default(),
            generator_name: crate::std_msgs::msg::String::default(),
        }
    }
}

impl ros2_client::Message for GeneratorInfo {}
