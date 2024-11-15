use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRegionsOfInterest3DReq {
    pub region_of_interest_ids: Vec<::std::string::String>,
}

impl Default for DeleteRegionsOfInterest3DReq {
    fn default() -> Self {
        DeleteRegionsOfInterest3DReq {
            region_of_interest_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for DeleteRegionsOfInterest3DReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRegionsOfInterest3DRes {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteRegionsOfInterest3DRes {
    fn default() -> Self {
        DeleteRegionsOfInterest3DRes {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for DeleteRegionsOfInterest3DRes {}


pub struct DeleteRegionsOfInterest3D;
impl ros2_client::Service for DeleteRegionsOfInterest3D {
    type Request = DeleteRegionsOfInterest3DReq;
    type Response = DeleteRegionsOfInterest3DRes;

    fn request_type_name(&self) -> &str { "DeleteRegionsOfInterest3DReq" }
    fn response_type_name(&self) -> &str { "DeleteRegionsOfInterest3DRes" }
}
