use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnEntityReq {
    pub entity_factory: crate::ros_gz_interfaces::msg::EntityFactory,
}

impl Default for SpawnEntityReq {
    fn default() -> Self {
        SpawnEntityReq {
            entity_factory: crate::ros_gz_interfaces::msg::EntityFactory::default(),
        }
    }
}

impl ros2_client::Message for SpawnEntityReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnEntityRes {
    pub success: bool,
}

impl Default for SpawnEntityRes {
    fn default() -> Self {
        SpawnEntityRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SpawnEntityRes {}


pub struct SpawnEntity;
impl ros2_client::Service for SpawnEntity {
    type Request = SpawnEntityReq;
    type Response = SpawnEntityRes;

    fn request_type_name(&self) -> &str { "SpawnEntityReq" }
    fn response_type_name(&self) -> &str { "SpawnEntityRes" }
}
