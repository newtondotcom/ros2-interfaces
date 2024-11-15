use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOctomapReq {

}

impl Default for GetOctomapReq {
    fn default() -> Self {
        GetOctomapReq {

        }
    }
}

impl ros2_client::Message for GetOctomapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOctomapRes {
    pub map: crate::octomap_msgs::msg::Octomap,
}

impl Default for GetOctomapRes {
    fn default() -> Self {
        GetOctomapRes {
            map: crate::octomap_msgs::msg::Octomap::default(),
        }
    }
}

impl ros2_client::Message for GetOctomapRes {}


pub struct GetOctomap;
impl ros2_client::Service for GetOctomap {
    type Request = GetOctomapReq;
    type Response = GetOctomapRes;

    fn request_type_name(&self) -> &str { "GetOctomapReq" }
    fn response_type_name(&self) -> &str { "GetOctomapRes" }
}
