use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResourceRequest {
    pub path: ::std::string::String,
    pub etag: ::std::string::String,
}

impl Default for GetResourceRequest {
    fn default() -> Self {
        GetResourceRequest {
            path: ::std::string::String::new(),
            etag: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetResourceRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResourceResponse {
    pub status_code: i32,
    pub error_reason: ::std::string::String,
    pub expanded_path: ::std::string::String,
    pub etag: ::std::string::String,
    pub body: Vec<u8>,
}

impl GetResourceResponse {
    pub const ERROR: i32 = 0;
    pub const OK: i32 = 1;
    pub const NOT_MODIFIED: i32 = 2;
}

impl Default for GetResourceResponse {
    fn default() -> Self {
        GetResourceResponse {
            status_code: 0,
            error_reason: ::std::string::String::new(),
            expanded_path: ::std::string::String::new(),
            etag: ::std::string::String::new(),
            body: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetResourceResponse {}


pub struct GetResource;
impl ros2_client::Service for GetResource {
    type Request = GetResourceRequest;
    type Response = GetResourceResponse;

    fn request_type_name(&self) -> &str { "GetResourceRequest" }
    fn response_type_name(&self) -> &str { "GetResourceResponse" }
}
