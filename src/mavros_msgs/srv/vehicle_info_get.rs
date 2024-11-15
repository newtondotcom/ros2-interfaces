use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleInfoGetReq {
    pub sysid: u8,
    pub compid: u8,
    pub get_all: bool,
}

impl VehicleInfoGetReq {
    pub const GET_MY_SYSID: u8 = 0;
    pub const GET_MY_COMPID: u8 = 0;
}

impl Default for VehicleInfoGetReq {
    fn default() -> Self {
        VehicleInfoGetReq {
            sysid: 0,
            compid: 0,
            get_all: false,
        }
    }
}

impl ros2_client::Message for VehicleInfoGetReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleInfoGetRes {
    pub success: bool,
    pub vehicles: Vec<crate::mavros_msgs::msg::VehicleInfo>,
}

impl Default for VehicleInfoGetRes {
    fn default() -> Self {
        VehicleInfoGetRes {
            success: false,
            vehicles: Vec::new(),
        }
    }
}

impl ros2_client::Message for VehicleInfoGetRes {}


pub struct VehicleInfoGet;
impl ros2_client::Service for VehicleInfoGet {
    type Request = VehicleInfoGetReq;
    type Response = VehicleInfoGetRes;

    fn request_type_name(&self) -> &str { "VehicleInfoGetReq" }
    fn response_type_name(&self) -> &str { "VehicleInfoGetRes" }
}
