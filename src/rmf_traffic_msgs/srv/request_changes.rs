use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestChangesReq {
    pub query_id: u64,
    pub version: u64,
    pub full_update: bool, // default: false
}

impl Default for RequestChangesReq {
    fn default() -> Self {
        RequestChangesReq {
            query_id: 0,
            version: 0,
            full_update: false,
        }
    }
}

impl ros2_client::Message for RequestChangesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestChangesRes {
    pub node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity,
    pub result: u8,
    pub error: ::std::string::String,
}

impl RequestChangesRes {
    pub const REQUEST_ACCEPTED: u8 = 1;
    pub const UNKNOWN_QUERY_ID: u8 = 2;
    pub const ERROR: u8 = 3;
}

impl Default for RequestChangesRes {
    fn default() -> Self {
        RequestChangesRes {
            node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity::default(),
            result: 0,
            error: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RequestChangesRes {}


pub struct RequestChanges;
impl ros2_client::Service for RequestChanges {
    type Request = RequestChangesReq;
    type Response = RequestChangesRes;

    fn request_type_name(&self) -> &str { "RequestChangesReq" }
    fn response_type_name(&self) -> &str { "RequestChangesRes" }
}
