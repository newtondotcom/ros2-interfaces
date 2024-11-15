use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearCostmapExceptRegionReq {
    pub reset_distance: f32,
}

impl Default for ClearCostmapExceptRegionReq {
    fn default() -> Self {
        ClearCostmapExceptRegionReq {
            reset_distance: 0.0,
        }
    }
}

impl ros2_client::Message for ClearCostmapExceptRegionReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearCostmapExceptRegionRes {
    pub response: crate::std_msgs::msg::Empty,
}

impl Default for ClearCostmapExceptRegionRes {
    fn default() -> Self {
        ClearCostmapExceptRegionRes {
            response: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for ClearCostmapExceptRegionRes {}


pub struct ClearCostmapExceptRegion;
impl ros2_client::Service for ClearCostmapExceptRegion {
    type Request = ClearCostmapExceptRegionReq;
    type Response = ClearCostmapExceptRegionRes;

    fn request_type_name(&self) -> &str { "ClearCostmapExceptRegionReq" }
    fn response_type_name(&self) -> &str { "ClearCostmapExceptRegionRes" }
}
