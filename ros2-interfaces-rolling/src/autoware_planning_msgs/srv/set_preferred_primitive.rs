use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPreferredPrimitiveRequest {
    pub preferred_primitives: Vec<crate::autoware_planning_msgs::msg::LaneletPrimitive>,
    pub reset: bool,
    pub uuid: crate::unique_identifier_msgs::msg::UUID,
}

impl Default for SetPreferredPrimitiveRequest {
    fn default() -> Self {
        SetPreferredPrimitiveRequest {
            preferred_primitives: Vec::new(),
            reset: false,
            uuid: crate::unique_identifier_msgs::msg::UUID::default(),
        }
    }
}

impl ros2_client::Message for SetPreferredPrimitiveRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPreferredPrimitiveResponse {
    pub status: crate::autoware_common_msgs::msg::ResponseStatus,
}

impl Default for SetPreferredPrimitiveResponse {
    fn default() -> Self {
        SetPreferredPrimitiveResponse {
            status: crate::autoware_common_msgs::msg::ResponseStatus::default(),
        }
    }
}

impl ros2_client::Message for SetPreferredPrimitiveResponse {}


pub struct SetPreferredPrimitive;
impl ros2_client::Service for SetPreferredPrimitive {
    type Request = SetPreferredPrimitiveRequest;
    type Response = SetPreferredPrimitiveResponse;

    fn request_type_name(&self) -> &str { "SetPreferredPrimitiveRequest" }
    fn response_type_name(&self) -> &str { "SetPreferredPrimitiveResponse" }
}
