use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPoseReq {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetPoseReq {
    fn default() -> Self {
        SetPoseReq {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

impl ros2_client::Message for SetPoseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPoseRes {

}

impl Default for SetPoseRes {
    fn default() -> Self {
        SetPoseRes {

        }
    }
}

impl ros2_client::Message for SetPoseRes {}


pub struct SetPose;
impl ros2_client::Service for SetPose {
    type Request = SetPoseReq;
    type Response = SetPoseRes;

    fn request_type_name(&self) -> &str { "SetPoseReq" }
    fn response_type_name(&self) -> &str { "SetPoseRes" }
}
