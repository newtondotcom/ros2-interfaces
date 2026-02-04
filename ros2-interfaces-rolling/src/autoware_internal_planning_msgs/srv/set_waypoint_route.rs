use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetWaypointRouteRequest {
    pub header: crate::std_msgs::msg::Header,
    pub goal_pose: crate::geometry_msgs::msg::Pose,
    pub waypoints: Vec<crate::geometry_msgs::msg::Pose>,
    pub uuid: crate::unique_identifier_msgs::msg::UUID,
    pub allow_modification: bool,
}

impl Default for SetWaypointRouteRequest {
    fn default() -> Self {
        SetWaypointRouteRequest {
            header: crate::std_msgs::msg::Header::default(),
            goal_pose: crate::geometry_msgs::msg::Pose::default(),
            waypoints: Vec::new(),
            uuid: crate::unique_identifier_msgs::msg::UUID::default(),
            allow_modification: false,
        }
    }
}

impl ros2_client::Message for SetWaypointRouteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetWaypointRouteResponse {
    pub status: crate::autoware_common_msgs::msg::ResponseStatus,
}

impl Default for SetWaypointRouteResponse {
    fn default() -> Self {
        SetWaypointRouteResponse {
            status: crate::autoware_common_msgs::msg::ResponseStatus::default(),
        }
    }
}

impl ros2_client::Message for SetWaypointRouteResponse {}


pub struct SetWaypointRoute;
impl ros2_client::Service for SetWaypointRoute {
    type Request = SetWaypointRouteRequest;
    type Response = SetWaypointRouteResponse;

    fn request_type_name(&self) -> &str { "SetWaypointRouteRequest" }
    fn response_type_name(&self) -> &str { "SetWaypointRouteResponse" }
}
