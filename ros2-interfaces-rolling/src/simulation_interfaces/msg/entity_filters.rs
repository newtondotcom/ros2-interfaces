use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityFilters {
    pub filter: ::std::string::String,
    pub categories: Vec<crate::simulation_interfaces::msg::EntityCategory>,
    pub tags: crate::simulation_interfaces::msg::TagsFilter,
    pub bounds: crate::simulation_interfaces::msg::Bounds,
}

impl Default for EntityFilters {
    fn default() -> Self {
        EntityFilters {
            filter: ::std::string::String::new(),
            categories: Vec::new(),
            tags: crate::simulation_interfaces::msg::TagsFilter::default(),
            bounds: crate::simulation_interfaces::msg::Bounds::default(),
        }
    }
}

impl ros2_client::Message for EntityFilters {}
