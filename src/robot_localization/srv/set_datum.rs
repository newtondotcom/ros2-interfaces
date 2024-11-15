use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDatumReq {
    pub geo_pose: crate::geographic_msgs::msg::GeoPose,
}

impl Default for SetDatumReq {
    fn default() -> Self {
        SetDatumReq {
            geo_pose: crate::geographic_msgs::msg::GeoPose::default(),
        }
    }
}

impl ros2_client::Message for SetDatumReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDatumRes {

}

impl Default for SetDatumRes {
    fn default() -> Self {
        SetDatumRes {

        }
    }
}

impl ros2_client::Message for SetDatumRes {}


pub struct SetDatum;
impl ros2_client::Service for SetDatum {
    type Request = SetDatumReq;
    type Response = SetDatumRes;

    fn request_type_name(&self) -> &str { "SetDatumReq" }
    fn response_type_name(&self) -> &str { "SetDatumRes" }
}
