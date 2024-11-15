use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectedMapsInfoReq {
    pub projected_maps_info: Vec<crate::map_msgs::msg::ProjectedMapInfo>,
}

impl Default for ProjectedMapsInfoReq {
    fn default() -> Self {
        ProjectedMapsInfoReq {
            projected_maps_info: Vec::new(),
        }
    }
}

impl ros2_client::Message for ProjectedMapsInfoReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectedMapsInfoRes {

}

impl Default for ProjectedMapsInfoRes {
    fn default() -> Self {
        ProjectedMapsInfoRes {

        }
    }
}

impl ros2_client::Message for ProjectedMapsInfoRes {}


pub struct ProjectedMapsInfo;
impl ros2_client::Service for ProjectedMapsInfo {
    type Request = ProjectedMapsInfoReq;
    type Response = ProjectedMapsInfoRes;

    fn request_type_name(&self) -> &str { "ProjectedMapsInfoReq" }
    fn response_type_name(&self) -> &str { "ProjectedMapsInfoRes" }
}
