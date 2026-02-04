use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowPathRequest {
    pub path_source: i32,
    pub path_name: ::std::string::String,
    pub path_file_location: ::std::string::String,
    pub path_follower_type: i32,
    pub robot_cmd_topic: ::std::string::String,
}

impl FollowPathRequest {
    pub const FOLLOW_PATH_FROM_ROS_TOPIC: i32 = 0;
    pub const FOLLOW_PATH_FROM_XML_FILE: i32 = 1;
}

impl Default for FollowPathRequest {
    fn default() -> Self {
        FollowPathRequest {
            path_source: 0,
            path_name: ::std::string::String::new(),
            path_file_location: ::std::string::String::new(),
            path_follower_type: 0,
            robot_cmd_topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FollowPathRequest {}
