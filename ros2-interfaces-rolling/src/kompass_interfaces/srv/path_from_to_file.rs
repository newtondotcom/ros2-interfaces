use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathFromToFileRequest {
    pub file_location: ::std::string::String,
    pub file_name: ::std::string::String,
}

impl Default for PathFromToFileRequest {
    fn default() -> Self {
        PathFromToFileRequest {
            file_location: ::std::string::String::new(),
            file_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for PathFromToFileRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathFromToFileResponse {
    pub path_length: f64,
    pub path_num_points: i32,
}

impl Default for PathFromToFileResponse {
    fn default() -> Self {
        PathFromToFileResponse {
            path_length: 0.0,
            path_num_points: 0,
        }
    }
}

impl ros2_client::Message for PathFromToFileResponse {}


pub struct PathFromToFile;
impl ros2_client::Service for PathFromToFile {
    type Request = PathFromToFileRequest;
    type Response = PathFromToFileResponse;

    fn request_type_name(&self) -> &str { "PathFromToFileRequest" }
    fn response_type_name(&self) -> &str { "PathFromToFileResponse" }
}
