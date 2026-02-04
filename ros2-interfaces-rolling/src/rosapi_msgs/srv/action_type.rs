use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionTypeRequest {
    pub action: ::std::string::String,
}

impl Default for ActionTypeRequest {
    fn default() -> Self {
        ActionTypeRequest {
            action: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ActionTypeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionTypeResponse {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for ActionTypeResponse {
    fn default() -> Self {
        ActionTypeResponse {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ActionTypeResponse {}


pub struct ActionType;
impl ros2_client::Service for ActionType {
    type Request = ActionTypeRequest;
    type Response = ActionTypeResponse;

    fn request_type_name(&self) -> &str { "ActionTypeRequest" }
    fn response_type_name(&self) -> &str { "ActionTypeResponse" }
}
