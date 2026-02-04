use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEntityRequest {
    pub entity: ::std::string::String,
}

impl Default for DeleteEntityRequest {
    fn default() -> Self {
        DeleteEntityRequest {
            entity: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteEntityRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEntityResponse {
    pub result: crate::simulation_interfaces::msg::Result,
}

impl Default for DeleteEntityResponse {
    fn default() -> Self {
        DeleteEntityResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
        }
    }
}

impl ros2_client::Message for DeleteEntityResponse {}


pub struct DeleteEntity;
impl ros2_client::Service for DeleteEntity {
    type Request = DeleteEntityRequest;
    type Response = DeleteEntityResponse;

    fn request_type_name(&self) -> &str { "DeleteEntityRequest" }
    fn response_type_name(&self) -> &str { "DeleteEntityResponse" }
}
