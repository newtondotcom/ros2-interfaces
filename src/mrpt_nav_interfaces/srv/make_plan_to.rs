use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakePlanToReq {
    pub target: crate::geometry_msgs::msg::PoseStamped,
}

impl Default for MakePlanToReq {
    fn default() -> Self {
        MakePlanToReq {
            target: crate::geometry_msgs::msg::PoseStamped::default(),
        }
    }
}

impl ros2_client::Message for MakePlanToReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakePlanToRes {
    pub valid_path_found: bool,
    pub waypoints: crate::mrpt_msgs::msg::WaypointSequence,
}

impl Default for MakePlanToRes {
    fn default() -> Self {
        MakePlanToRes {
            valid_path_found: false,
            waypoints: crate::mrpt_msgs::msg::WaypointSequence::default(),
        }
    }
}

impl ros2_client::Message for MakePlanToRes {}


pub struct MakePlanTo;
impl ros2_client::Service for MakePlanTo {
    type Request = MakePlanToReq;
    type Response = MakePlanToRes;

    fn request_type_name(&self) -> &str { "MakePlanToReq" }
    fn response_type_name(&self) -> &str { "MakePlanToRes" }
}
