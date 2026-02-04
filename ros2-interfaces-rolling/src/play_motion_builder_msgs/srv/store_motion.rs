use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoreMotionRequest {
    pub file_path: ::std::string::String,
    pub ros_name: ::std::string::String,
    pub meta: crate::play_motion_builder_msgs::msg::Meta,
}

impl Default for StoreMotionRequest {
    fn default() -> Self {
        StoreMotionRequest {
            file_path: ::std::string::String::new(),
            ros_name: ::std::string::String::new(),
            meta: crate::play_motion_builder_msgs::msg::Meta::default(),
        }
    }
}

impl ros2_client::Message for StoreMotionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoreMotionResponse {
    pub ok: bool,
    pub message: ::std::string::String,
}

impl Default for StoreMotionResponse {
    fn default() -> Self {
        StoreMotionResponse {
            ok: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StoreMotionResponse {}


pub struct StoreMotion;
impl ros2_client::Service for StoreMotion {
    type Request = StoreMotionRequest;
    type Response = StoreMotionResponse;

    fn request_type_name(&self) -> &str { "StoreMotionRequest" }
    fn response_type_name(&self) -> &str { "StoreMotionResponse" }
}
