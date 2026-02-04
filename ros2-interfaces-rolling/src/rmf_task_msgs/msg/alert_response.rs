use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertResponse {
    pub id: ::std::string::String,
    pub response: ::std::string::String,
}

impl Default for AlertResponse {
    fn default() -> Self {
        AlertResponse {
            id: ::std::string::String::new(),
            response: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AlertResponse {}
