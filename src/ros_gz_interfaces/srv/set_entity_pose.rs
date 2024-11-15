use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEntityPoseReq {
    pub entity: crate::ros_gz_interfaces::msg::Entity,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for SetEntityPoseReq {
    fn default() -> Self {
        SetEntityPoseReq {
            entity: crate::ros_gz_interfaces::msg::Entity::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for SetEntityPoseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEntityPoseRes {
    pub success: bool,
}

impl Default for SetEntityPoseRes {
    fn default() -> Self {
        SetEntityPoseRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SetEntityPoseRes {}


pub struct SetEntityPose;
impl ros2_client::Service for SetEntityPose {
    type Request = SetEntityPoseReq;
    type Response = SetEntityPoseRes;

    fn request_type_name(&self) -> &str { "SetEntityPoseReq" }
    fn response_type_name(&self) -> &str { "SetEntityPoseRes" }
}
