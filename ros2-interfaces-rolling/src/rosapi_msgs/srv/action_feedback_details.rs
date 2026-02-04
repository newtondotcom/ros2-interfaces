use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionFeedbackDetailsRequest {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for ActionFeedbackDetailsRequest {
    fn default() -> Self {
        ActionFeedbackDetailsRequest {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ActionFeedbackDetailsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionFeedbackDetailsResponse {
    pub typedefs: Vec<crate::rosapi_msgs::msg::TypeDef>,
}

impl Default for ActionFeedbackDetailsResponse {
    fn default() -> Self {
        ActionFeedbackDetailsResponse {
            typedefs: Vec::new(),
        }
    }
}

impl ros2_client::Message for ActionFeedbackDetailsResponse {}


pub struct ActionFeedbackDetails;
impl ros2_client::Service for ActionFeedbackDetails {
    type Request = ActionFeedbackDetailsRequest;
    type Response = ActionFeedbackDetailsResponse;

    fn request_type_name(&self) -> &str { "ActionFeedbackDetailsRequest" }
    fn response_type_name(&self) -> &str { "ActionFeedbackDetailsResponse" }
}
