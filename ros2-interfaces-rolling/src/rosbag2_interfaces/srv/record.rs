use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordRequest {
    pub uri: ::std::string::String,
}

impl Default for RecordRequest {
    fn default() -> Self {
        RecordRequest {
            uri: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RecordRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordResponse {
    pub return_code: i32,
    pub error_string: ::std::string::String,
}

impl Default for RecordResponse {
    fn default() -> Self {
        RecordResponse {
            return_code: 0,
            error_string: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RecordResponse {}


pub struct Record;
impl ros2_client::Service for Record {
    type Request = RecordRequest;
    type Response = RecordResponse;

    fn request_type_name(&self) -> &str { "RecordRequest" }
    fn response_type_name(&self) -> &str { "RecordResponse" }
}
