use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRegionsOfInterest2DReq {
    pub region_of_interest_2d_ids: Vec<::std::string::String>,
}

impl Default for DeleteRegionsOfInterest2DReq {
    fn default() -> Self {
        DeleteRegionsOfInterest2DReq {
            region_of_interest_2d_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for DeleteRegionsOfInterest2DReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRegionsOfInterest2DRes {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteRegionsOfInterest2DRes {
    fn default() -> Self {
        DeleteRegionsOfInterest2DRes {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for DeleteRegionsOfInterest2DRes {}


pub struct DeleteRegionsOfInterest2D;
impl ros2_client::Service for DeleteRegionsOfInterest2D {
    type Request = DeleteRegionsOfInterest2DReq;
    type Response = DeleteRegionsOfInterest2DRes;

    fn request_type_name(&self) -> &str { "DeleteRegionsOfInterest2DReq" }
    fn response_type_name(&self) -> &str { "DeleteRegionsOfInterest2DRes" }
}
