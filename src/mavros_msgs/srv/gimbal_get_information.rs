use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalGetInformationReq {

}

impl Default for GimbalGetInformationReq {
    fn default() -> Self {
        GimbalGetInformationReq {

        }
    }
}

impl ros2_client::Message for GimbalGetInformationReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalGetInformationRes {
    pub success: bool,
    pub result: u8,
}

impl Default for GimbalGetInformationRes {
    fn default() -> Self {
        GimbalGetInformationRes {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for GimbalGetInformationRes {}


pub struct GimbalGetInformation;
impl ros2_client::Service for GimbalGetInformation {
    type Request = GimbalGetInformationReq;
    type Response = GimbalGetInformationRes;

    fn request_type_name(&self) -> &str { "GimbalGetInformationReq" }
    fn response_type_name(&self) -> &str { "GimbalGetInformationRes" }
}
