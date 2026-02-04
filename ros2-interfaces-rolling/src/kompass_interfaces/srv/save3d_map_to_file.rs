use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Save3dMapToFileRequest {
    pub topic_name: ::std::string::String,
    pub save_file_location: ::std::string::String,
    pub save_file_name: ::std::string::String,
    pub timeout: f64,
}

impl Default for Save3dMapToFileRequest {
    fn default() -> Self {
        Save3dMapToFileRequest {
            topic_name: ::std::string::String::new(),
            save_file_location: ::std::string::String::new(),
            save_file_name: ::std::string::String::new(),
            timeout: 0.0,
        }
    }
}

impl ros2_client::Message for Save3dMapToFileRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Save3dMapToFileResponse {
    pub success: bool,
}

impl Default for Save3dMapToFileResponse {
    fn default() -> Self {
        Save3dMapToFileResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for Save3dMapToFileResponse {}


pub struct Save3dMapToFile;
impl ros2_client::Service for Save3dMapToFile {
    type Request = Save3dMapToFileRequest;
    type Response = Save3dMapToFileResponse;

    fn request_type_name(&self) -> &str { "Save3dMapToFileRequest" }
    fn response_type_name(&self) -> &str { "Save3dMapToFileResponse" }
}
