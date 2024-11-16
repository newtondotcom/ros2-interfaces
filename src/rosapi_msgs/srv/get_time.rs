use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTimeReq {

}

impl Default for GetTimeReq {
    fn default() -> Self {
        GetTimeReq {

        }
    }
}

impl ros2_client::Message for GetTimeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTimeRes {
    pub time: crate::builtin_interfaces::msg::Time,
}

impl Default for GetTimeRes {
    fn default() -> Self {
        GetTimeRes {
            time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for GetTimeRes {}


pub struct GetTime;
impl ros2_client::Service for GetTime {
    type Request = GetTimeReq;
    type Response = GetTimeRes;

    fn request_type_name(&self) -> &str { "GetTimeReq" }
    fn response_type_name(&self) -> &str { "GetTimeRes" }
}
