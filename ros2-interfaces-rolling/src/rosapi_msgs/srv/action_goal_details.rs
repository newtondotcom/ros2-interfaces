use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionGoalDetailsRequest {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for ActionGoalDetailsRequest {
    fn default() -> Self {
        ActionGoalDetailsRequest {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ActionGoalDetailsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionGoalDetailsResponse {
    pub typedefs: Vec<crate::rosapi_msgs::msg::TypeDef>,
}

impl Default for ActionGoalDetailsResponse {
    fn default() -> Self {
        ActionGoalDetailsResponse {
            typedefs: Vec::new(),
        }
    }
}

impl ros2_client::Message for ActionGoalDetailsResponse {}


pub struct ActionGoalDetails;
impl ros2_client::Service for ActionGoalDetails {
    type Request = ActionGoalDetailsRequest;
    type Response = ActionGoalDetailsResponse;

    fn request_type_name(&self) -> &str { "ActionGoalDetailsRequest" }
    fn response_type_name(&self) -> &str { "ActionGoalDetailsResponse" }
}
