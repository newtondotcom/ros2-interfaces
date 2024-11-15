use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsPathValidReq {
    pub path: crate::nav_msgs::msg::Path,
}

impl Default for IsPathValidReq {
    fn default() -> Self {
        IsPathValidReq {
            path: crate::nav_msgs::msg::Path::default(),
        }
    }
}

impl ros2_client::Message for IsPathValidReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsPathValidRes {
    pub is_valid: bool,
    pub invalid_pose_indices: Vec<i32>,
}

impl Default for IsPathValidRes {
    fn default() -> Self {
        IsPathValidRes {
            is_valid: false,
            invalid_pose_indices: Vec::new(),
        }
    }
}

impl ros2_client::Message for IsPathValidRes {}


pub struct IsPathValid;
impl ros2_client::Service for IsPathValid {
    type Request = IsPathValidReq;
    type Response = IsPathValidRes;

    fn request_type_name(&self) -> &str { "IsPathValidReq" }
    fn response_type_name(&self) -> &str { "IsPathValidRes" }
}
