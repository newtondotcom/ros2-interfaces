use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotSoftwareVersionReq {

}

impl Default for GetRobotSoftwareVersionReq {
    fn default() -> Self {
        GetRobotSoftwareVersionReq {

        }
    }
}

impl ros2_client::Message for GetRobotSoftwareVersionReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotSoftwareVersionRes {
    pub major: u32,
    pub minor: u32,
    pub bugfix: u32,
    pub build: u32,
}

impl Default for GetRobotSoftwareVersionRes {
    fn default() -> Self {
        GetRobotSoftwareVersionRes {
            major: 0,
            minor: 0,
            bugfix: 0,
            build: 0,
        }
    }
}

impl ros2_client::Message for GetRobotSoftwareVersionRes {}


pub struct GetRobotSoftwareVersion;
impl ros2_client::Service for GetRobotSoftwareVersion {
    type Request = GetRobotSoftwareVersionReq;
    type Response = GetRobotSoftwareVersionRes;

    fn request_type_name(&self) -> &str { "GetRobotSoftwareVersionReq" }
    fn response_type_name(&self) -> &str { "GetRobotSoftwareVersionRes" }
}
