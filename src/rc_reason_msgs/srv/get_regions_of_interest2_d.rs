use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRegionsOfInterest2DReq {
    pub region_of_interest_2d_ids: Vec<::std::string::String>,
}

impl Default for GetRegionsOfInterest2DReq {
    fn default() -> Self {
        GetRegionsOfInterest2DReq {
            region_of_interest_2d_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetRegionsOfInterest2DReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRegionsOfInterest2DRes {
    pub regions_of_interest: Vec<crate::rc_reason_msgs::msg::RegionOfInterest2D>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for GetRegionsOfInterest2DRes {
    fn default() -> Self {
        GetRegionsOfInterest2DRes {
            regions_of_interest: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for GetRegionsOfInterest2DRes {}


pub struct GetRegionsOfInterest2D;
impl ros2_client::Service for GetRegionsOfInterest2D {
    type Request = GetRegionsOfInterest2DReq;
    type Response = GetRegionsOfInterest2DRes;

    fn request_type_name(&self) -> &str { "GetRegionsOfInterest2DReq" }
    fn response_type_name(&self) -> &str { "GetRegionsOfInterest2DRes" }
}
