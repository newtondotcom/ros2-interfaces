use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetPoseReq {
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for ResetPoseReq {
    fn default() -> Self {
        ResetPoseReq {
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for ResetPoseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetPoseRes {

}

impl Default for ResetPoseRes {
    fn default() -> Self {
        ResetPoseRes {

        }
    }
}

impl ros2_client::Message for ResetPoseRes {}


pub struct ResetPose;
impl ros2_client::Service for ResetPose {
    type Request = ResetPoseReq;
    type Response = ResetPoseRes;

    fn request_type_name(&self) -> &str { "ResetPoseReq" }
    fn response_type_name(&self) -> &str { "ResetPoseRes" }
}
