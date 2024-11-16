use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRegionOfInterest2DReq {
    pub region_of_interest_2d: crate::rc_reason_msgs::msg::RegionOfInterest2D,
}

impl Default for SetRegionOfInterest2DReq {
    fn default() -> Self {
        SetRegionOfInterest2DReq {
            region_of_interest_2d: crate::rc_reason_msgs::msg::RegionOfInterest2D::default(),
        }
    }
}

impl ros2_client::Message for SetRegionOfInterest2DReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRegionOfInterest2DRes {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for SetRegionOfInterest2DRes {
    fn default() -> Self {
        SetRegionOfInterest2DRes {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for SetRegionOfInterest2DRes {}


pub struct SetRegionOfInterest2D;
impl ros2_client::Service for SetRegionOfInterest2D {
    type Request = SetRegionOfInterest2DReq;
    type Response = SetRegionOfInterest2DRes;

    fn request_type_name(&self) -> &str { "SetRegionOfInterest2DReq" }
    fn response_type_name(&self) -> &str { "SetRegionOfInterest2DRes" }
}
