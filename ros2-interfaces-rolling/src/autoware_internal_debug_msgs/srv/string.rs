use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringRequest {
    pub data: ::std::string::String,
}

impl Default for StringRequest {
    fn default() -> Self {
        StringRequest {
            data: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StringRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringResponse {
    pub success: bool,
}

impl Default for StringResponse {
    fn default() -> Self {
        StringResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for StringResponse {}


pub struct String;
impl ros2_client::Service for String {
    type Request = StringRequest;
    type Response = StringResponse;

    fn request_type_name(&self) -> &str { "StringRequest" }
    fn response_type_name(&self) -> &str { "StringResponse" }
}
