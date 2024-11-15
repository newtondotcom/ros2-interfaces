use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalizedImageCropReq {
    pub top_left: crate::geometry_msgs::msg::Pose2D,
    pub bottom_right: crate::geometry_msgs::msg::Pose2D,
}

impl Default for NormalizedImageCropReq {
    fn default() -> Self {
        NormalizedImageCropReq {
            top_left: crate::geometry_msgs::msg::Pose2D::default(),
            bottom_right: crate::geometry_msgs::msg::Pose2D::default(),
        }
    }
}

impl ros2_client::Message for NormalizedImageCropReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalizedImageCropRes {
    pub status: i64,
}

impl Default for NormalizedImageCropRes {
    fn default() -> Self {
        NormalizedImageCropRes {
            status: 0,
        }
    }
}

impl ros2_client::Message for NormalizedImageCropRes {}


pub struct NormalizedImageCrop;
impl ros2_client::Service for NormalizedImageCrop {
    type Request = NormalizedImageCropReq;
    type Response = NormalizedImageCropRes;

    fn request_type_name(&self) -> &str { "NormalizedImageCropReq" }
    fn response_type_name(&self) -> &str { "NormalizedImageCropRes" }
}
