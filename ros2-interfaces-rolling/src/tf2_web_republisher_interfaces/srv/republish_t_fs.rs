use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepublishTFsRequest {
    pub source_frames: Vec<::std::string::String>,
    pub target_frame: ::std::string::String,
    pub angular_thres: f32,
    pub trans_thres: f32,
    pub rate: f32,
    pub timeout: crate::builtin_interfaces::msg::Duration,
}

impl Default for RepublishTFsRequest {
    fn default() -> Self {
        RepublishTFsRequest {
            source_frames: Vec::new(),
            target_frame: ::std::string::String::new(),
            angular_thres: 0.0,
            trans_thres: 0.0,
            rate: 0.0,
            timeout: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for RepublishTFsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepublishTFsResponse {
    pub topic_name: ::std::string::String,
}

impl Default for RepublishTFsResponse {
    fn default() -> Self {
        RepublishTFsResponse {
            topic_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RepublishTFsResponse {}


pub struct RepublishTFs;
impl ros2_client::Service for RepublishTFs {
    type Request = RepublishTFsRequest;
    type Response = RepublishTFsResponse;

    fn request_type_name(&self) -> &str { "RepublishTFsRequest" }
    fn response_type_name(&self) -> &str { "RepublishTFsResponse" }
}
