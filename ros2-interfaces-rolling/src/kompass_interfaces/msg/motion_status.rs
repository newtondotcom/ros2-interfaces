use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotionStatus {
    pub status: u8,
    pub message: ::std::string::String,
}

impl MotionStatus {
    pub const STATUS_HEALTHY: u8 = 0;
    pub const STATUS_FREEZING_ROBOT: u8 = 1;
    pub const STATUS_LOW_LEVEL_ERROR: u8 = 2;
    pub const STATUS_CONTROLLER_ERROR: u8 = 3;
    pub const STATUS_PATH_FOLLOWER_ERROR: u8 = 4;
    pub const STATUS_ODOMETRY_NOT_RECEIVED: u8 = 5;
}

impl Default for MotionStatus {
    fn default() -> Self {
        MotionStatus {
            status: 0,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MotionStatus {}
