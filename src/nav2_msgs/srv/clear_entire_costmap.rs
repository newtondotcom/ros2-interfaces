use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearEntireCostmapReq {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for ClearEntireCostmapReq {
    fn default() -> Self {
        ClearEntireCostmapReq {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for ClearEntireCostmapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearEntireCostmapRes {
    pub response: crate::std_msgs::msg::Empty,
}

impl Default for ClearEntireCostmapRes {
    fn default() -> Self {
        ClearEntireCostmapRes {
            response: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for ClearEntireCostmapRes {}


pub struct ClearEntireCostmap;
impl ros2_client::Service for ClearEntireCostmap {
    type Request = ClearEntireCostmapReq;
    type Response = ClearEntireCostmapRes;

    fn request_type_name(&self) -> &str { "ClearEntireCostmapReq" }
    fn response_type_name(&self) -> &str { "ClearEntireCostmapRes" }
}
