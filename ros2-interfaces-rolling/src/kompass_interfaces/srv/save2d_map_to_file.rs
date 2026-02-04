use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Save2dMapToFileRequest {
    pub topic_name: ::std::string::String,
    pub save_file_location: ::std::string::String,
    pub save_file_name: ::std::string::String,
    pub timeout: f64,
    pub free_thresh: f64,
    pub occupied_thresh: f64,
}

impl Default for Save2dMapToFileRequest {
    fn default() -> Self {
        Save2dMapToFileRequest {
            topic_name: ::std::string::String::new(),
            save_file_location: ::std::string::String::new(),
            save_file_name: ::std::string::String::new(),
            timeout: 0.0,
            free_thresh: 0.0,
            occupied_thresh: 0.0,
        }
    }
}

impl ros2_client::Message for Save2dMapToFileRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Save2dMapToFileResponse {
    pub success: bool,
}

impl Default for Save2dMapToFileResponse {
    fn default() -> Self {
        Save2dMapToFileResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for Save2dMapToFileResponse {}


pub struct Save2dMapToFile;
impl ros2_client::Service for Save2dMapToFile {
    type Request = Save2dMapToFileRequest;
    type Response = Save2dMapToFileResponse;

    fn request_type_name(&self) -> &str { "Save2dMapToFileRequest" }
    fn response_type_name(&self) -> &str { "Save2dMapToFileResponse" }
}
