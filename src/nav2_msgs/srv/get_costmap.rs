use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCostmapReq {
    pub specs: crate::nav2_msgs::msg::CostmapMetaData,
}

impl Default for GetCostmapReq {
    fn default() -> Self {
        GetCostmapReq {
            specs: crate::nav2_msgs::msg::CostmapMetaData::default(),
        }
    }
}

impl ros2_client::Message for GetCostmapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCostmapRes {
    pub map: crate::nav2_msgs::msg::Costmap,
}

impl Default for GetCostmapRes {
    fn default() -> Self {
        GetCostmapRes {
            map: crate::nav2_msgs::msg::Costmap::default(),
        }
    }
}

impl ros2_client::Message for GetCostmapRes {}


pub struct GetCostmap;
impl ros2_client::Service for GetCostmap {
    type Request = GetCostmapReq;
    type Response = GetCostmapRes;

    fn request_type_name(&self) -> &str { "GetCostmapReq" }
    fn response_type_name(&self) -> &str { "GetCostmapRes" }
}
