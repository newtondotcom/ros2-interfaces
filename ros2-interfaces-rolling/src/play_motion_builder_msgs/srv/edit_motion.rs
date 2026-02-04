use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditMotionRequest {
    pub step_id: u16,
    pub action: u8,
    pub time: f32,
}

impl EditMotionRequest {
    pub const LIST: u8 = 0;
    pub const APPEND: u8 = 1;
    pub const EDIT: u8 = 2;
    pub const COPY_AS_NEXT: u8 = 3;
    pub const COPY_AS_LAST: u8 = 4;
    pub const REMOVE: u8 = 5;
    pub const EDIT_TIME: u8 = 6;
}

impl Default for EditMotionRequest {
    fn default() -> Self {
        EditMotionRequest {
            step_id: 0,
            action: 0,
            time: 0.0,
        }
    }
}

impl ros2_client::Message for EditMotionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditMotionResponse {
    pub ok: bool,
    pub message: ::std::string::String,
    pub motion: crate::play_motion_builder_msgs::msg::Motion,
}

impl Default for EditMotionResponse {
    fn default() -> Self {
        EditMotionResponse {
            ok: false,
            message: ::std::string::String::new(),
            motion: crate::play_motion_builder_msgs::msg::Motion::default(),
        }
    }
}

impl ros2_client::Message for EditMotionResponse {}


pub struct EditMotion;
impl ros2_client::Service for EditMotion {
    type Request = EditMotionRequest;
    type Response = EditMotionResponse;

    fn request_type_name(&self) -> &str { "EditMotionRequest" }
    fn response_type_name(&self) -> &str { "EditMotionResponse" }
}
