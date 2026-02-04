use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spawnable {
    pub entity_resource: crate::simulation_interfaces::msg::Resource,
    pub description: ::std::string::String,
    pub spawn_bounds: crate::simulation_interfaces::msg::Bounds,
}

impl Default for Spawnable {
    fn default() -> Self {
        Spawnable {
            entity_resource: crate::simulation_interfaces::msg::Resource::default(),
            description: ::std::string::String::new(),
            spawn_bounds: crate::simulation_interfaces::msg::Bounds::default(),
        }
    }
}

impl ros2_client::Message for Spawnable {}
