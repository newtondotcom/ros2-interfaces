use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetUTMZoneReq {
    pub utm_zone: ::std::string::String,
}

impl Default for SetUTMZoneReq {
    fn default() -> Self {
        SetUTMZoneReq {
            utm_zone: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetUTMZoneReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetUTMZoneRes {

}

impl Default for SetUTMZoneRes {
    fn default() -> Self {
        SetUTMZoneRes {

        }
    }
}

impl ros2_client::Message for SetUTMZoneRes {}


pub struct SetUTMZone;
impl ros2_client::Service for SetUTMZone {
    type Request = SetUTMZoneReq;
    type Response = SetUTMZoneRes;

    fn request_type_name(&self) -> &str { "SetUTMZoneReq" }
    fn response_type_name(&self) -> &str { "SetUTMZoneRes" }
}
