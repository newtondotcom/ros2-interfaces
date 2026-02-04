use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListClipsenvsRequest {

}

impl Default for ListClipsenvsRequest {
    fn default() -> Self {
        ListClipsenvsRequest {

        }
    }
}

impl ros2_client::Message for ListClipsenvsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListClipsenvsResponse {
    pub success: bool,
    pub plugins: Vec<::std::string::String>,
    pub error: ::std::string::String,
}

impl Default for ListClipsenvsResponse {
    fn default() -> Self {
        ListClipsenvsResponse {
            success: false,
            plugins: Vec::new(),
            error: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ListClipsenvsResponse {}


pub struct ListClipsenvs;
impl ros2_client::Service for ListClipsenvs {
    type Request = ListClipsenvsRequest;
    type Response = ListClipsenvsResponse;

    fn request_type_name(&self) -> &str { "ListClipsenvsRequest" }
    fn response_type_name(&self) -> &str { "ListClipsenvsResponse" }
}
