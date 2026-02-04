use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigureFromFileRequest {
    pub path_to_file: ::std::string::String,
    pub keep_alive: bool,
}

impl Default for ConfigureFromFileRequest {
    fn default() -> Self {
        ConfigureFromFileRequest {
            path_to_file: ::std::string::String::new(),
            keep_alive: false,
        }
    }
}

impl ros2_client::Message for ConfigureFromFileRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigureFromFileResponse {
    pub success: bool,
    pub error_msg: ::std::string::String,
}

impl Default for ConfigureFromFileResponse {
    fn default() -> Self {
        ConfigureFromFileResponse {
            success: false,
            error_msg: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ConfigureFromFileResponse {}


pub struct ConfigureFromFile;
impl ros2_client::Service for ConfigureFromFile {
    type Request = ConfigureFromFileRequest;
    type Response = ConfigureFromFileResponse;

    fn request_type_name(&self) -> &str { "ConfigureFromFileRequest" }
    fn response_type_name(&self) -> &str { "ConfigureFromFileResponse" }
}
