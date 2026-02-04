use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionResultDetailsRequest {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for ActionResultDetailsRequest {
    fn default() -> Self {
        ActionResultDetailsRequest {
            type_: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ActionResultDetailsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionResultDetailsResponse {
    pub typedefs: Vec<crate::rosapi_msgs::msg::TypeDef>,
}

impl Default for ActionResultDetailsResponse {
    fn default() -> Self {
        ActionResultDetailsResponse {
            typedefs: Vec::new(),
        }
    }
}

impl ros2_client::Message for ActionResultDetailsResponse {}


pub struct ActionResultDetails;
impl ros2_client::Service for ActionResultDetails {
    type Request = ActionResultDetailsRequest;
    type Response = ActionResultDetailsResponse;

    fn request_type_name(&self) -> &str { "ActionResultDetailsRequest" }
    fn response_type_name(&self) -> &str { "ActionResultDetailsResponse" }
}
