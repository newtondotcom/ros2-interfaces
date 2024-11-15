use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInitialPoseReq {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetInitialPoseReq {
    fn default() -> Self {
        SetInitialPoseReq {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

impl ros2_client::Message for SetInitialPoseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInitialPoseRes {

}

impl Default for SetInitialPoseRes {
    fn default() -> Self {
        SetInitialPoseRes {

        }
    }
}

impl ros2_client::Message for SetInitialPoseRes {}


pub struct SetInitialPose;
impl ros2_client::Service for SetInitialPose {
    type Request = SetInitialPoseReq;
    type Response = SetInitialPoseRes;

    fn request_type_name(&self) -> &str { "SetInitialPoseReq" }
    fn response_type_name(&self) -> &str { "SetInitialPoseRes" }
}
