use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeAutowareControlRequest {
    pub autoware_control: bool,
}

impl Default for ChangeAutowareControlRequest {
    fn default() -> Self {
        ChangeAutowareControlRequest {
            autoware_control: false,
        }
    }
}

impl ros2_client::Message for ChangeAutowareControlRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeAutowareControlResponse {
    pub status: crate::autoware_common_msgs::msg::ResponseStatus,
}

impl Default for ChangeAutowareControlResponse {
    fn default() -> Self {
        ChangeAutowareControlResponse {
            status: crate::autoware_common_msgs::msg::ResponseStatus::default(),
        }
    }
}

impl ros2_client::Message for ChangeAutowareControlResponse {}


pub struct ChangeAutowareControl;
impl ros2_client::Service for ChangeAutowareControl {
    type Request = ChangeAutowareControlRequest;
    type Response = ChangeAutowareControlResponse;

    fn request_type_name(&self) -> &str { "ChangeAutowareControlRequest" }
    fn response_type_name(&self) -> &str { "ChangeAutowareControlResponse" }
}
