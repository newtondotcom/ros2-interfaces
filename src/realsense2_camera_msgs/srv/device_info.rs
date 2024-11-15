use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceInfoReq {

}

impl Default for DeviceInfoReq {
    fn default() -> Self {
        DeviceInfoReq {

        }
    }
}

impl ros2_client::Message for DeviceInfoReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceInfoRes {
    pub device_name: ::std::string::String,
    pub serial_number: ::std::string::String,
    pub firmware_version: ::std::string::String,
    pub usb_type_descriptor: ::std::string::String,
    pub firmware_update_id: ::std::string::String,
    pub sensors: ::std::string::String,
    pub physical_port: ::std::string::String,
}

impl Default for DeviceInfoRes {
    fn default() -> Self {
        DeviceInfoRes {
            device_name: ::std::string::String::new(),
            serial_number: ::std::string::String::new(),
            firmware_version: ::std::string::String::new(),
            usb_type_descriptor: ::std::string::String::new(),
            firmware_update_id: ::std::string::String::new(),
            sensors: ::std::string::String::new(),
            physical_port: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeviceInfoRes {}


pub struct DeviceInfo;
impl ros2_client::Service for DeviceInfo {
    type Request = DeviceInfoReq;
    type Response = DeviceInfoRes;

    fn request_type_name(&self) -> &str { "DeviceInfoReq" }
    fn response_type_name(&self) -> &str { "DeviceInfoRes" }
}
