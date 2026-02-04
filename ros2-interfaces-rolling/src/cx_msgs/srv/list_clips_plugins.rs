use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListClipsPluginsRequest {
    pub env_name: ::std::string::String,
}

impl Default for ListClipsPluginsRequest {
    fn default() -> Self {
        ListClipsPluginsRequest {
            env_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ListClipsPluginsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListClipsPluginsResponse {
    pub success: bool,
    pub plugins: Vec<::std::string::String>,
    pub error: ::std::string::String,
}

impl Default for ListClipsPluginsResponse {
    fn default() -> Self {
        ListClipsPluginsResponse {
            success: false,
            plugins: Vec::new(),
            error: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ListClipsPluginsResponse {}


pub struct ListClipsPlugins;
impl ros2_client::Service for ListClipsPlugins {
    type Request = ListClipsPluginsRequest;
    type Response = ListClipsPluginsResponse;

    fn request_type_name(&self) -> &str { "ListClipsPluginsRequest" }
    fn response_type_name(&self) -> &str { "ListClipsPluginsResponse" }
}
