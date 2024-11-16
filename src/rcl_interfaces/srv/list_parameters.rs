use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListParametersReq {
    pub prefixes: Vec<::std::string::String>,
    pub depth: u64,
}

impl ListParametersReq {
    pub const DEPTH_RECURSIVE: u64 = 0;
}

impl Default for ListParametersReq {
    fn default() -> Self {
        ListParametersReq {
            prefixes: Vec::new(),
            depth: 0,
        }
    }
}

impl ros2_client::Message for ListParametersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListParametersRes {
    pub result: crate::rcl_interfaces::msg::ListParametersResult,
}

impl Default for ListParametersRes {
    fn default() -> Self {
        ListParametersRes {
            result: crate::rcl_interfaces::msg::ListParametersResult::default(),
        }
    }
}

impl ros2_client::Message for ListParametersRes {}


pub struct ListParameters;
impl ros2_client::Service for ListParameters {
    type Request = ListParametersReq;
    type Response = ListParametersRes;

    fn request_type_name(&self) -> &str { "ListParametersReq" }
    fn response_type_name(&self) -> &str { "ListParametersRes" }
}
