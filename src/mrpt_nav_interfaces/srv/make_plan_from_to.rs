use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakePlanFromToReq {
    pub start: crate::geometry_msgs::msg::Pose,
    pub target: crate::geometry_msgs::msg::Pose,
}

impl Default for MakePlanFromToReq {
    fn default() -> Self {
        MakePlanFromToReq {
            start: crate::geometry_msgs::msg::Pose::default(),
            target: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for MakePlanFromToReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakePlanFromToRes {
    pub valid_path_found: bool,
    pub waypoints: crate::mrpt_msgs::msg::WaypointSequence,
}

impl Default for MakePlanFromToRes {
    fn default() -> Self {
        MakePlanFromToRes {
            valid_path_found: false,
            waypoints: crate::mrpt_msgs::msg::WaypointSequence::default(),
        }
    }
}

impl ros2_client::Message for MakePlanFromToRes {}


pub struct MakePlanFromTo;
impl ros2_client::Service for MakePlanFromTo {
    type Request = MakePlanFromToReq;
    type Response = MakePlanFromToRes;

    fn request_type_name(&self) -> &str { "MakePlanFromToReq" }
    fn response_type_name(&self) -> &str { "MakePlanFromToRes" }
}
