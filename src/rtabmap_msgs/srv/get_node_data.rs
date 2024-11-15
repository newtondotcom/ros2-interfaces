use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeDataReq {
    pub ids: Vec<i32>,
    pub images: bool,
    pub scan: bool,
    pub grid: bool,
    pub user_data: bool,
}

impl Default for GetNodeDataReq {
    fn default() -> Self {
        GetNodeDataReq {
            ids: Vec::new(),
            images: false,
            scan: false,
            grid: false,
            user_data: false,
        }
    }
}

impl ros2_client::Message for GetNodeDataReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeDataRes {
    pub data: Vec<crate::rtabmap_msgs::msg::Node>,
}

impl Default for GetNodeDataRes {
    fn default() -> Self {
        GetNodeDataRes {
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetNodeDataRes {}


pub struct GetNodeData;
impl ros2_client::Service for GetNodeData {
    type Request = GetNodeDataReq;
    type Response = GetNodeDataRes;

    fn request_type_name(&self) -> &str { "GetNodeDataReq" }
    fn response_type_name(&self) -> &str { "GetNodeDataRes" }
}
