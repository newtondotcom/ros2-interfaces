use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetROSVersionReq {

}

impl Default for GetROSVersionReq {
    fn default() -> Self {
        GetROSVersionReq {

        }
    }
}

impl ros2_client::Message for GetROSVersionReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetROSVersionRes {
    pub version: u8,
    pub distro: ::std::string::String,
}

impl Default for GetROSVersionRes {
    fn default() -> Self {
        GetROSVersionRes {
            version: 0,
            distro: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetROSVersionRes {}


pub struct GetROSVersion;
impl ros2_client::Service for GetROSVersion {
    type Request = GetROSVersionReq;
    type Response = GetROSVersionRes;

    fn request_type_name(&self) -> &str { "GetROSVersionReq" }
    fn response_type_name(&self) -> &str { "GetROSVersionRes" }
}
