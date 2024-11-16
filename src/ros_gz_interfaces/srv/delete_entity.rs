use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEntityReq {
    pub entity: crate::ros_gz_interfaces::msg::Entity,
}

impl Default for DeleteEntityReq {
    fn default() -> Self {
        DeleteEntityReq {
            entity: crate::ros_gz_interfaces::msg::Entity::default(),
        }
    }
}

impl ros2_client::Message for DeleteEntityReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEntityRes {
    pub success: bool,
}

impl Default for DeleteEntityRes {
    fn default() -> Self {
        DeleteEntityRes {
            success: false,
        }
    }
}

impl ros2_client::Message for DeleteEntityRes {}


pub struct DeleteEntity;
impl ros2_client::Service for DeleteEntity {
    type Request = DeleteEntityReq;
    type Response = DeleteEntityRes;

    fn request_type_name(&self) -> &str { "DeleteEntityReq" }
    fn response_type_name(&self) -> &str { "DeleteEntityRes" }
}
