use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityInfo {
    pub category: crate::simulation_interfaces::msg::EntityCategory,
    pub description: ::std::string::String,
    pub tags: Vec<::std::string::String>,
}

impl Default for EntityInfo {
    fn default() -> Self {
        EntityInfo {
            category: crate::simulation_interfaces::msg::EntityCategory::default(),
            description: ::std::string::String::new(),
            tags: Vec::new(),
        }
    }
}

impl ros2_client::Message for EntityInfo {}
