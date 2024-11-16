use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSelectedPointCloudMapReq {
    pub cell_ids: Vec<::std::string::String>,
}

impl Default for GetSelectedPointCloudMapReq {
    fn default() -> Self {
        GetSelectedPointCloudMapReq {
            cell_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetSelectedPointCloudMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSelectedPointCloudMapRes {
    pub header: crate::std_msgs::msg::Header,
    pub new_pointcloud_with_ids: Vec<crate::autoware_map_msgs::msg::PointCloudMapCellWithID>,
}

impl Default for GetSelectedPointCloudMapRes {
    fn default() -> Self {
        GetSelectedPointCloudMapRes {
            header: crate::std_msgs::msg::Header::default(),
            new_pointcloud_with_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetSelectedPointCloudMapRes {}


pub struct GetSelectedPointCloudMap;
impl ros2_client::Service for GetSelectedPointCloudMap {
    type Request = GetSelectedPointCloudMapReq;
    type Response = GetSelectedPointCloudMapRes;

    fn request_type_name(&self) -> &str { "GetSelectedPointCloudMapReq" }
    fn response_type_name(&self) -> &str { "GetSelectedPointCloudMapRes" }
}
