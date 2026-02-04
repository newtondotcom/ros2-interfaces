use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLaneletRouteRequest {
    pub header: crate::std_msgs::msg::Header,
    pub goal_pose: crate::geometry_msgs::msg::Pose,
    pub segments: Vec<crate::autoware_planning_msgs::msg::LaneletSegment>,
    pub uuid: crate::unique_identifier_msgs::msg::UUID,
    pub allow_modification: bool,
}

impl Default for SetLaneletRouteRequest {
    fn default() -> Self {
        SetLaneletRouteRequest {
            header: crate::std_msgs::msg::Header::default(),
            goal_pose: crate::geometry_msgs::msg::Pose::default(),
            segments: Vec::new(),
            uuid: crate::unique_identifier_msgs::msg::UUID::default(),
            allow_modification: false,
        }
    }
}

impl ros2_client::Message for SetLaneletRouteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLaneletRouteResponse {
    pub status: crate::autoware_common_msgs::msg::ResponseStatus,
}

impl Default for SetLaneletRouteResponse {
    fn default() -> Self {
        SetLaneletRouteResponse {
            status: crate::autoware_common_msgs::msg::ResponseStatus::default(),
        }
    }
}

impl ros2_client::Message for SetLaneletRouteResponse {}


pub struct SetLaneletRoute;
impl ros2_client::Service for SetLaneletRoute {
    type Request = SetLaneletRouteRequest;
    type Response = SetLaneletRouteResponse;

    fn request_type_name(&self) -> &str { "SetLaneletRouteRequest" }
    fn response_type_name(&self) -> &str { "SetLaneletRouteResponse" }
}
