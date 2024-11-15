use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectTagsReq {
    pub tags: Vec<crate::rc_reason_msgs::msg::Tag>,
    pub pose_frame: ::std::string::String,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
}

impl Default for DetectTagsReq {
    fn default() -> Self {
        DetectTagsReq {
            tags: Vec::new(),
            pose_frame: ::std::string::String::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for DetectTagsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectTagsRes {
    pub tags: Vec<crate::rc_reason_msgs::msg::DetectedTag>,
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DetectTagsRes {
    fn default() -> Self {
        DetectTagsRes {
            tags: Vec::new(),
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for DetectTagsRes {}


pub struct DetectTags;
impl ros2_client::Service for DetectTags {
    type Request = DetectTagsReq;
    type Response = DetectTagsRes;

    fn request_type_name(&self) -> &str { "DetectTagsReq" }
    fn response_type_name(&self) -> &str { "DetectTagsRes" }
}
