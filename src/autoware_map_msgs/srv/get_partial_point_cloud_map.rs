use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPartialPointCloudMapReq {
    pub area: crate::autoware_map_msgs::msg::AreaInfo,
}

impl Default for GetPartialPointCloudMapReq {
    fn default() -> Self {
        GetPartialPointCloudMapReq {
            area: crate::autoware_map_msgs::msg::AreaInfo::default(),
        }
    }
}

impl ros2_client::Message for GetPartialPointCloudMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPartialPointCloudMapRes {
    pub header: crate::std_msgs::msg::Header,
    pub new_pointcloud_with_ids: Vec<crate::autoware_map_msgs::msg::PointCloudMapCellWithID>,
}

impl Default for GetPartialPointCloudMapRes {
    fn default() -> Self {
        GetPartialPointCloudMapRes {
            header: crate::std_msgs::msg::Header::default(),
            new_pointcloud_with_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetPartialPointCloudMapRes {}


pub struct GetPartialPointCloudMap;
impl ros2_client::Service for GetPartialPointCloudMap {
    type Request = GetPartialPointCloudMapReq;
    type Response = GetPartialPointCloudMapRes;

    fn request_type_name(&self) -> &str { "GetPartialPointCloudMapReq" }
    fn response_type_name(&self) -> &str { "GetPartialPointCloudMapRes" }
}
