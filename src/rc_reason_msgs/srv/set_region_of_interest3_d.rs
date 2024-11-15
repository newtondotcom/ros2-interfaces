use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRegionOfInterest3DReq {
    pub region_of_interest: crate::rc_reason_msgs::msg::RegionOfInterest3D,
}

impl Default for SetRegionOfInterest3DReq {
    fn default() -> Self {
        SetRegionOfInterest3DReq {
            region_of_interest: crate::rc_reason_msgs::msg::RegionOfInterest3D::default(),
        }
    }
}

impl ros2_client::Message for SetRegionOfInterest3DReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRegionOfInterest3DRes {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for SetRegionOfInterest3DRes {
    fn default() -> Self {
        SetRegionOfInterest3DRes {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for SetRegionOfInterest3DRes {}


pub struct SetRegionOfInterest3D;
impl ros2_client::Service for SetRegionOfInterest3D {
    type Request = SetRegionOfInterest3DReq;
    type Response = SetRegionOfInterest3DRes;

    fn request_type_name(&self) -> &str { "SetRegionOfInterest3DReq" }
    fn response_type_name(&self) -> &str { "SetRegionOfInterest3DRes" }
}
