use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePointcloudOctomapReq {
    pub cloud: crate::sensor_msgs::msg::PointCloud2,
}

impl Default for UpdatePointcloudOctomapReq {
    fn default() -> Self {
        UpdatePointcloudOctomapReq {
            cloud: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}

impl ros2_client::Message for UpdatePointcloudOctomapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePointcloudOctomapRes {
    pub success: bool,
}

impl Default for UpdatePointcloudOctomapRes {
    fn default() -> Self {
        UpdatePointcloudOctomapRes {
            success: false,
        }
    }
}

impl ros2_client::Message for UpdatePointcloudOctomapRes {}


pub struct UpdatePointcloudOctomap;
impl ros2_client::Service for UpdatePointcloudOctomap {
    type Request = UpdatePointcloudOctomapReq;
    type Response = UpdatePointcloudOctomapRes;

    fn request_type_name(&self) -> &str { "UpdatePointcloudOctomapReq" }
    fn response_type_name(&self) -> &str { "UpdatePointcloudOctomapRes" }
}
