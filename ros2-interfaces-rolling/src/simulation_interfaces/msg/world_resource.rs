use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldResource {
    pub name: ::std::string::String,
    pub world_resource: crate::simulation_interfaces::msg::Resource,
    pub description: ::std::string::String,
    pub tags: Vec<::std::string::String>,
}

impl Default for WorldResource {
    fn default() -> Self {
        WorldResource {
            name: ::std::string::String::new(),
            world_resource: crate::simulation_interfaces::msg::Resource::default(),
            description: ::std::string::String::new(),
            tags: Vec::new(),
        }
    }
}

impl ros2_client::Message for WorldResource {}
