use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPoseDeprecatedReq {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetPoseDeprecatedReq {
    fn default() -> Self {
        SetPoseDeprecatedReq {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

impl ros2_client::Message for SetPoseDeprecatedReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPoseDeprecatedRes {

}

impl Default for SetPoseDeprecatedRes {
    fn default() -> Self {
        SetPoseDeprecatedRes {

        }
    }
}

impl ros2_client::Message for SetPoseDeprecatedRes {}


pub struct SetPoseDeprecated;
impl ros2_client::Service for SetPoseDeprecated {
    type Request = SetPoseDeprecatedReq;
    type Response = SetPoseDeprecatedRes;

    fn request_type_name(&self) -> &str { "SetPoseDeprecatedReq" }
    fn response_type_name(&self) -> &str { "SetPoseDeprecatedRes" }
}
