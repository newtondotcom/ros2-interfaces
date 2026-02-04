use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnEntityRequest {
    pub name: ::std::string::String,
    pub allow_renaming: bool,
    pub entity_resource: crate::simulation_interfaces::msg::Resource,
    pub entity_namespace: ::std::string::String,
    pub initial_pose: crate::geometry_msgs::msg::PoseStamped,
}

impl Default for SpawnEntityRequest {
    fn default() -> Self {
        SpawnEntityRequest {
            name: ::std::string::String::new(),
            allow_renaming: false,
            entity_resource: crate::simulation_interfaces::msg::Resource::default(),
            entity_namespace: ::std::string::String::new(),
            initial_pose: crate::geometry_msgs::msg::PoseStamped::default(),
        }
    }
}

impl ros2_client::Message for SpawnEntityRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnEntityResponse {
    pub result: crate::simulation_interfaces::msg::Result,
    pub entity_name: ::std::string::String,
}

impl SpawnEntityResponse {
    pub const NAME_NOT_UNIQUE: u8 = 101;
    pub const NAME_INVALID: u8 = 102;
    pub const UNSUPPORTED_FORMAT: u8 = 103;
    pub const NO_RESOURCE: u8 = 104;
    pub const NAMESPACE_INVALID: u8 = 105;
    pub const RESOURCE_PARSE_ERROR: u8 = 106;
    pub const MISSING_ASSETS: u8 = 107;
    pub const UNSUPPORTED_ASSETS: u8 = 108;
    pub const INVALID_POSE: u8 = 109;
}

impl Default for SpawnEntityResponse {
    fn default() -> Self {
        SpawnEntityResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
            entity_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpawnEntityResponse {}


pub struct SpawnEntity;
impl ros2_client::Service for SpawnEntity {
    type Request = SpawnEntityRequest;
    type Response = SpawnEntityResponse;

    fn request_type_name(&self) -> &str { "SpawnEntityRequest" }
    fn response_type_name(&self) -> &str { "SpawnEntityResponse" }
}
