use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDifferentialPointCloudMapReq {
    pub area: crate::autoware_map_msgs::msg::AreaInfo,
    pub cached_ids: Vec<::std::string::String>,
}

impl Default for GetDifferentialPointCloudMapReq {
    fn default() -> Self {
        GetDifferentialPointCloudMapReq {
            area: crate::autoware_map_msgs::msg::AreaInfo::default(),
            cached_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetDifferentialPointCloudMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDifferentialPointCloudMapRes {
    pub header: crate::std_msgs::msg::Header,
    pub new_pointcloud_with_ids: Vec<crate::autoware_map_msgs::msg::PointCloudMapCellWithID>,
    pub ids_to_remove: Vec<::std::string::String>,
}

impl Default for GetDifferentialPointCloudMapRes {
    fn default() -> Self {
        GetDifferentialPointCloudMapRes {
            header: crate::std_msgs::msg::Header::default(),
            new_pointcloud_with_ids: Vec::new(),
            ids_to_remove: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetDifferentialPointCloudMapRes {}


pub struct GetDifferentialPointCloudMap;
impl ros2_client::Service for GetDifferentialPointCloudMap {
    type Request = GetDifferentialPointCloudMapReq;
    type Response = GetDifferentialPointCloudMapRes;

    fn request_type_name(&self) -> &str { "GetDifferentialPointCloudMapReq" }
    fn response_type_name(&self) -> &str { "GetDifferentialPointCloudMapRes" }
}
