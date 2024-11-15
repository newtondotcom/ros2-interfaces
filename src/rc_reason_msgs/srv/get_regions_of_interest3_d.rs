use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRegionsOfInterest3DReq {
    pub region_of_interest_ids: Vec<::std::string::String>,
}

impl Default for GetRegionsOfInterest3DReq {
    fn default() -> Self {
        GetRegionsOfInterest3DReq {
            region_of_interest_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetRegionsOfInterest3DReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRegionsOfInterest3DRes {
    pub regions_of_interest: Vec<crate::rc_reason_msgs::msg::RegionOfInterest3D>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for GetRegionsOfInterest3DRes {
    fn default() -> Self {
        GetRegionsOfInterest3DRes {
            regions_of_interest: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for GetRegionsOfInterest3DRes {}


pub struct GetRegionsOfInterest3D;
impl ros2_client::Service for GetRegionsOfInterest3D {
    type Request = GetRegionsOfInterest3DReq;
    type Response = GetRegionsOfInterest3DRes;

    fn request_type_name(&self) -> &str { "GetRegionsOfInterest3DReq" }
    fn response_type_name(&self) -> &str { "GetRegionsOfInterest3DRes" }
}
