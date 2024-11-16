use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoundReq {
    pub value: u8,
}

impl Default for SoundReq {
    fn default() -> Self {
        SoundReq {
            value: 0,
        }
    }
}

impl ros2_client::Message for SoundReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoundRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SoundRes {
    fn default() -> Self {
        SoundRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SoundRes {}


pub struct Sound;
impl ros2_client::Service for Sound {
    type Request = SoundReq;
    type Response = SoundRes;

    fn request_type_name(&self) -> &str { "SoundReq" }
    fn response_type_name(&self) -> &str { "SoundRes" }
}
