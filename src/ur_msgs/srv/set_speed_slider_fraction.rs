use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSpeedSliderFractionReq {
    pub speed_slider_fraction: f64,
}

impl Default for SetSpeedSliderFractionReq {
    fn default() -> Self {
        SetSpeedSliderFractionReq {
            speed_slider_fraction: 0.0,
        }
    }
}

impl ros2_client::Message for SetSpeedSliderFractionReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSpeedSliderFractionRes {
    pub success: bool,
}

impl Default for SetSpeedSliderFractionRes {
    fn default() -> Self {
        SetSpeedSliderFractionRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SetSpeedSliderFractionRes {}


pub struct SetSpeedSliderFraction;
impl ros2_client::Service for SetSpeedSliderFraction {
    type Request = SetSpeedSliderFractionReq;
    type Response = SetSpeedSliderFractionRes;

    fn request_type_name(&self) -> &str { "SetSpeedSliderFractionReq" }
    fn response_type_name(&self) -> &str { "SetSpeedSliderFractionRes" }
}
