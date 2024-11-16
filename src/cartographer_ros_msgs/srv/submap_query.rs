use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmapQueryReq {
    pub trajectory_id: i32,
    pub submap_index: i32,
}

impl Default for SubmapQueryReq {
    fn default() -> Self {
        SubmapQueryReq {
            trajectory_id: 0,
            submap_index: 0,
        }
    }
}

impl ros2_client::Message for SubmapQueryReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmapQueryRes {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
    pub submap_version: i32,
    pub textures: Vec<crate::cartographer_ros_msgs::msg::SubmapTexture>,
}

impl Default for SubmapQueryRes {
    fn default() -> Self {
        SubmapQueryRes {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
            submap_version: 0,
            textures: Vec::new(),
        }
    }
}

impl ros2_client::Message for SubmapQueryRes {}


pub struct SubmapQuery;
impl ros2_client::Service for SubmapQuery {
    type Request = SubmapQueryReq;
    type Response = SubmapQueryRes;

    fn request_type_name(&self) -> &str { "SubmapQueryReq" }
    fn response_type_name(&self) -> &str { "SubmapQueryRes" }
}
