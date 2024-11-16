use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocalizeNearPoseReq {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for RelocalizeNearPoseReq {
    fn default() -> Self {
        RelocalizeNearPoseReq {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

impl ros2_client::Message for RelocalizeNearPoseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocalizeNearPoseRes {
    pub accepted: bool,
}

impl Default for RelocalizeNearPoseRes {
    fn default() -> Self {
        RelocalizeNearPoseRes {
            accepted: false,
        }
    }
}

impl ros2_client::Message for RelocalizeNearPoseRes {}


pub struct RelocalizeNearPose;
impl ros2_client::Service for RelocalizeNearPose {
    type Request = RelocalizeNearPoseReq;
    type Response = RelocalizeNearPoseRes;

    fn request_type_name(&self) -> &str { "RelocalizeNearPoseReq" }
    fn response_type_name(&self) -> &str { "RelocalizeNearPoseRes" }
}
