use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalBundleAdjustmentReq {
    #[serde(rename = "type")]    pub type_: i32,
    pub iterations: i32,
    pub pixel_variance: f32,
    pub voc_matches: bool,
}

impl Default for GlobalBundleAdjustmentReq {
    fn default() -> Self {
        GlobalBundleAdjustmentReq {
            type_: 0,
            iterations: 0,
            pixel_variance: 0.0,
            voc_matches: false,
        }
    }
}

impl ros2_client::Message for GlobalBundleAdjustmentReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalBundleAdjustmentRes {

}

impl Default for GlobalBundleAdjustmentRes {
    fn default() -> Self {
        GlobalBundleAdjustmentRes {

        }
    }
}

impl ros2_client::Message for GlobalBundleAdjustmentRes {}


pub struct GlobalBundleAdjustment;
impl ros2_client::Service for GlobalBundleAdjustment {
    type Request = GlobalBundleAdjustmentReq;
    type Response = GlobalBundleAdjustmentRes;

    fn request_type_name(&self) -> &str { "GlobalBundleAdjustmentReq" }
    fn response_type_name(&self) -> &str { "GlobalBundleAdjustmentRes" }
}
