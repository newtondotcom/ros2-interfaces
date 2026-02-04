use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Motion {
    pub joints: Vec<::std::string::String>,
    pub keyframes: Vec<crate::play_motion_builder_msgs::msg::Frame>,
    pub used_group: ::std::string::String,
}

impl Default for Motion {
    fn default() -> Self {
        Motion {
            joints: Vec::new(),
            keyframes: Vec::new(),
            used_group: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Motion {}
